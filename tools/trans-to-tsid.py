# podman exec postgres_db_1 sh -c 'psql -v ON_ERROR_STOP=1 -U chnots -d chnotsprod < /tmp/chnots-prod-db-20250612-162658.psql'

tf = [
    ("chnot_metadata", "id"),
    ("chnot_record", "id"),
    ("chnot_record", "meta_id"),
    ("chnot_tag", "id"),
    ("chnot_tag", "chnot_meta_id"),
    ("k_file", "id"),
    ("inline_k_file", "id"),
    ("llm_chat_bot", "id"),
    ("llm_chat_template", "id"),
    ("llm_chat_session", "id"),
    ("llm_chat_session", "template_id"),
    ("llm_chat_record", "id"),
    ("llm_chat_record", "session_id"),
    ("llm_chat_record", "pre_record_id"),
    ("llm_chat_record", "role_id"),
]

tf2 = [("kkv", "key"), ("kkv", "value")]

def id_to_tsid(table):
    return f"select id as uid, (extract(epoch from insert_time) * 1000000 + random() * 1000)::numeric::int8 as tsid from {table}"

def update_from_ids(table, column):
    return f"update {table} a set {column} = b.tsid from ids b where b.uid = a.{column}"

def trans_column(table, column):
    return f"alter table {table} alter column {column} TYPE BIGINT USING ({column}::int8)"

sql1 = "create table ids as " + " union ".join([id_to_tsid(x[0]) for x in tf])
sql2 = ";\n".join([update_from_ids(x[0], x[1]) for x in tf])
sql3 = ";\n".join([trans_column(x[0], x[1]) for x in tf]) + ";\n".join([update_from_ids(x[0], x[1]) for x in tf2])


print(sql1, ";")
print(sql2, ";")
print(sql3, ";")
