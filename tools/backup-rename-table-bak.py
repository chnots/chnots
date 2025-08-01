from pathlib import Path


source_path = Path(__file__).resolve()
source_dir = source_path.parent

with open(f"{source_dir}/backuplib.py") as f:
    exec(f.read())


def rename_to_bak():
    import time

    def rename(cursor, table_name):
        if "bak" in table_name:
            print(
                f"alter table public.{table_name} rename to {table_name + '_' + str(int(time.time()))}; -- 1"
            )
        if "bak" not in table_name:
            t = table_name + "_bak"
            print(f"alter table public.{table_name} rename to {t}; -- 2")

    for_all_tables(rename)


rename_to_bak()
