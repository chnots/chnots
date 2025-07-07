import psycopg2
from psycopg2 import sql
from datetime import datetime

# Example usage
db_params = {
    "host": "localhost",
    "database": "chnotsdev",
    "user": "postgres",
    "password": "chnotsdev",
    "port": "5432",
}

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


def bak_to_new_schema():
    sname = f"chnotsprod_{datetime.now().strftime('%Y%m%d%H%M%S')}"
    print(sname)

    def bak_one(cursor, table_name):
        cursor.execute(
            sql.SQL("CREATE SCHEMA IF NOT EXISTS {}").format(sql.Identifier(sname))
        )
        # Create the table in new schema with same structure
        create = sql.SQL("""
            CREATE TABLE {}.{} (LIKE public.{} INCLUDING ALL)
        """).format(
            sql.Identifier(sname),
            sql.Identifier(table_name),
            sql.Identifier(table_name),
        )
        print(create)
        cursor.execute(create)

        # Copy data from public to new schema
        cursor.execute(
            sql.SQL("""
            INSERT INTO {}.{} 
            SELECT * FROM public.{}
        """).format(
                sql.Identifier(sname),
                sql.Identifier(table_name),
                sql.Identifier(table_name),
            )
        )

    for_all_tables(bak_one)


def rename_to_bak():
    def rename(cursor, table_name):
        if "_bak" in table_name:
            return
        print(table_name)
        try:
            cursor.execute(
                sql.SQL("""
                drop table public.{} 
            """).format(
                    sql.Identifier(table_name + "_bak"),
                )
            )
            cursor.execute(
                sql.SQL("""
                alter table public.{} 
                rename to {}
            """).format(
                    sql.Identifier(table_name),
                    sql.Identifier(table_name + "_bak"),
                )
            )
        except Exception as ex:  # noqa: E722
            print(ex)

    for_all_tables(rename)


def insert_to_from_bak():
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
        print(f"insert into {table_name}({af}) select {af} from {table_name}_bak;")
        print(f"-- {table_name}")

    for_all_tables(rename)

insert_to_from_bak()