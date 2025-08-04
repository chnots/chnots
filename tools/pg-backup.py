import psycopg2
from psycopg2 import sql
from datetime import datetime
import argparse
from enum import Enum


def for_all_tables(fn):
    """
    Creates a new schema and copies all tables from public schema to it.

    Args:
        db_params: Dictionary containing database connection parameters
        new_schema_name: Name of the new schema to create
    """
    try:
        # Connect to the database
        print(db_params)
        conn = psycopg2.connect(**db_params)
        cursor = conn.cursor()

        # Get all tables from public schema
        cursor.execute("""
            SELECT table_name 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
        """)
        tables = cursor.fetchall()
        print(tables)

        # Copy each table to the new schema
        for table in tables:
            table_name = table[0]
            fn(cursor, table_name)

        conn.commit()

    except Exception as e:
        conn.rollback()
        print(f"Error: {e}")
    finally:
        if conn:
            cursor.close()
            conn.close()


def as_string(composable):
    if isinstance(composable, sql.Composed):
        return "".join([as_string(x) for x in composable])
    elif isinstance(composable, sql.SQL):
        return composable.string
    else:
        rv = sql.ext.adapt(composable._wrapped).getquoted()
        return rv.decode() if isinstance(rv, bytes) else rv


def bak_to_new_schema(bak_schema=None):
    schema_name = (
        f"public_{datetime.now().strftime('%Y%m%d%H%M%S')}"
        if bak_schema is None
        else bak_schema
    )

    def bak_one(cursor, table_name):
        cursor.execute(
            sql.SQL("CREATE SCHEMA IF NOT EXISTS {}").format(
                sql.Identifier(schema_name)
            )
        )
        # Create the table in new schema with same structure
        create = sql.SQL("""
            CREATE TABLE {}.{} (LIKE public.{} INCLUDING ALL)
        """).format(
            sql.Identifier(schema_name),
            sql.Identifier(table_name),
            sql.Identifier(table_name),
        )
        print(f"create table {schema_name}.{table_name}")
        cursor.execute(create)

        # Copy data from public to new schema
        cursor.execute(
            sql.SQL("""
            INSERT INTO {}.{} 
            SELECT * FROM public.{}
        """).format(
                sql.Identifier(schema_name),
                sql.Identifier(table_name),
                sql.Identifier(table_name),
            )
        )

    for_all_tables(bak_one)


def insert_to_from_bak(bak_schema=None):
    if bak_schema is None:
        raise Exception("bak schema is None, we cannot recover from it.")
    bak_to_new_schema(None)

    def rename(cursor, table_name):
        if "_bak" in table_name:
            return
        cursor.execute(
            sql.SQL(f"""
            SELECT column_name
            FROM information_schema.columns
            WHERE table_name = '{table_name}' AND table_schema = 'public';
        """)
        )

        fields = cursor.fetchall()
        af = ", ".join([x[0] for x in fields])
        isql = f"insert into {table_name}({af}) select {af} from {bak_schema}.{table_name};"
        print(isql)
        cursor.execute(sql.SQL(isql))

    for_all_tables(rename)


class Operation(Enum):
    bak2Schema = ["bak2Schema", bak_to_new_schema]
    rename2bak = ["fromBakSchema", insert_to_from_bak]


parser = argparse.ArgumentParser()
parser.add_argument("-H", "--host", type=str, required=False, default="localhost")
parser.add_argument("-P", "--port", type=int, required=False, default=5432)
parser.add_argument("-p", "--password", type=str, required=True)
parser.add_argument("-d", "--database", type=str, required=True)
parser.add_argument("-u", "--user", type=str, required=True)
parser.add_argument("--bakschema", type=str, required=False)

parser.add_argument(
    "-o",
    "--operation",
    help=f"available operations: {[x.value[0] for x in Operation]}",
    type=str,
    required=True,
)
args = parser.parse_args()

# Example usage
db_params = {
    "host": args.host,
    "database": args.database,
    "user": args.user,
    "password": args.password,
    "port": args.port,
}

operated = False
for o in Operation:
    if args.operation.startswith(o.value[0]):
        operated = True
        o.value[1](args.bakschema)

if not operated:
    raise Exception(
        f"not valid operation, valid operations are: {[x.value[0] for x in Operation]}"
    )
