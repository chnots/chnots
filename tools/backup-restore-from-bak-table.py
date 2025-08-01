from psycopg2 import sql
from pathlib import Path


source_path = Path(__file__).resolve()
source_dir = source_path.parent

with open(f"{source_dir}/backuplib.py") as f:
    exec(f.read())


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