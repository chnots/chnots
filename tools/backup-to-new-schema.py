from pathlib import Path
from psycopg2 import sql
from datetime import datetime


source_path = Path(__file__).resolve()
source_dir = source_path.parent

with open(f"{source_dir}/backuplib.py") as f:
    exec(f.read())


def bak_to_new_schema():
    sname = f"public_{datetime.now().strftime('%Y%m%d%H%M%S')}"
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


bak_to_new_schema()
