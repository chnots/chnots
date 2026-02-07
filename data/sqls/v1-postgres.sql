CREATE TABLE chnot_kind_rel (
  meta_otid int8 NOT NULL,
  kind_id varchar(200) NOT NULL,
  tid int8 NOT NULL
);

CREATE TABLE chnot_kind_rel_hist (
  meta_otid int8 NOT NULL,
  kind_id varchar(200) NOT NULL,
  tid int8 NOT NULL
);

CREATE INDEX chnot_kind_rel_hist_meta_otid ON chnot_kind_rel_hist USING btree (meta_otid);

CREATE UNIQUE INDEX chnot_kind_rel_hist_ukey_tid ON chnot_kind_rel_hist USING btree (tid);

CREATE UNIQUE INDEX chnot_kind_rel_pkey1 ON chnot_kind_rel USING btree (meta_otid);

CREATE UNIQUE INDEX chnot_kind_rel_ukey_tid ON chnot_kind_rel USING btree (tid);

CREATE TABLE chnot_metadata (
  otid int8 NOT NULL,
  kspace varchar(40) NOT NULL,
  kind varchar(40) NOT NULL,
  pin_time timestamptz,
  archive_time timestamptz,
  tid int8 NOT NULL
);

CREATE TABLE chnot_metadata_hist (
  otid int8 NOT NULL,
  kspace varchar(40) NOT NULL,
  kind varchar(40) NOT NULL,
  pin_time timestamptz,
  archive_time timestamptz,
  tid int8 NOT NULL
);

CREATE INDEX chnot_metadata_hist_otid ON chnot_metadata_hist USING btree (otid);

CREATE UNIQUE INDEX chnot_metadata_hist_ukey_tid ON chnot_metadata_hist USING btree (tid);

CREATE UNIQUE INDEX chnot_metadata_pkey1 ON chnot_metadata USING btree (otid);

CREATE UNIQUE INDEX chnot_metadata_ukey_tid ON chnot_metadata USING btree (tid);

CREATE TABLE chnot_record (
  meta_otid int8 NOT NULL,
  tid int8 NOT NULL,
  todo_event varchar(20),
  content text NOT NULL,
  archor bool NOT NULL
);

CREATE TABLE chnot_record_hist (
  meta_otid int8 NOT NULL,
  tid int8 NOT NULL,
  todo_event varchar(20),
  content text NOT NULL,
  archor bool NOT NULL
);

CREATE INDEX chnot_record_hist_meta_otid ON chnot_record_hist USING btree (meta_otid);

CREATE UNIQUE INDEX chnot_record_hist_ukey_tid ON chnot_record_hist USING btree (tid);

CREATE UNIQUE INDEX chnot_record_pkey1 ON chnot_record USING btree (meta_otid);

CREATE UNIQUE INDEX chnot_record_ukey_tid ON chnot_record USING btree (tid);

CREATE TABLE chnot_tag (
  tag varchar(800) NOT NULL,
  meta_otid int8 NOT NULL,
  kspace varchar(40) NOT NULL,
  tid int8 NOT NULL
);

CREATE TABLE chnot_tag_hist (
  tag varchar(800) NOT NULL,
  meta_otid int8 NOT NULL,
  kspace varchar(40) NOT NULL,
  tid int8 NOT NULL
);

CREATE INDEX chnot_tag_hist_meta_otid ON chnot_tag_hist USING btree (meta_otid);

CREATE INDEX chnot_tag_hist_tag ON chnot_tag_hist USING btree (tag);

CREATE UNIQUE INDEX chnot_tag_hist_ukey_tid ON chnot_tag_hist USING btree (tid);

CREATE UNIQUE INDEX chnot_tag_pkey1 ON chnot_tag USING btree (tag, meta_otid);

CREATE UNIQUE INDEX chnot_tag_ukey_tid ON chnot_tag USING btree (tid);

CREATE TABLE inline_k_file (
  sid varchar(100) NOT NULL,
  tid int8 NOT NULL,
  content text NOT NULL
);

CREATE UNIQUE INDEX inline_k_file_pkey1 ON inline_k_file USING btree (sid);

CREATE UNIQUE INDEX inline_k_file_ukey_tid ON inline_k_file USING btree (tid);

CREATE TABLE k_file_meta (
  id varchar(100) NOT NULL,
  inline bool NOT NULL,
  archor bool NOT NULL,
  tid int8 NOT NULL,
  filename varchar(1024) NOT NULL,
  content_type varchar(200) NOT NULL,
  last_modified int8 NOT NULL,
  sid varchar(100) NOT NULL,
  filesize int8 NOT NULL
);

CREATE TABLE k_file_meta_hist (
  id varchar(100) NOT NULL,
  inline bool NOT NULL,
  archor bool NOT NULL,
  tid int8 NOT NULL,
  filename varchar(1024) NOT NULL,
  content_type varchar(200) NOT NULL,
  last_modified int8 NOT NULL,
  sid varchar(100) NOT NULL,
  filesize int8 NOT NULL
);

CREATE INDEX k_file_meta_hist_id ON k_file_meta_hist USING btree (id);

CREATE UNIQUE INDEX k_file_meta_hist_ukey_tid ON k_file_meta_hist USING btree (tid);

CREATE UNIQUE INDEX k_file_meta_pkey1 ON k_file_meta USING btree (id);

CREATE UNIQUE INDEX k_file_meta_ukey_tid ON k_file_meta USING btree (tid);

CREATE TABLE k_space (
  name varchar(500) NOT NULL,
  color varchar(100) NOT NULL,
  managers text NOT NULL,
  tid int8 NOT NULL,
  public_access bool NOT NULL
);

CREATE TABLE k_space_hist (
  name varchar(500) NOT NULL,
  color varchar(100) NOT NULL,
  managers text NOT NULL,
  tid int8 NOT NULL,
  public_access bool NOT NULL
);

CREATE INDEX k_space_hist_name ON k_space_hist USING btree (name);

CREATE UNIQUE INDEX k_space_hist_ukey_tid ON k_space_hist USING btree (tid);

CREATE UNIQUE INDEX k_space_pkey ON k_space USING btree (name);

CREATE UNIQUE INDEX k_space_ukey_tid ON k_space USING btree (tid);

CREATE TABLE k_tab_cell_date (
  table_otid int8 NOT NULL,
  col_otid int8 NOT NULL,
  row_otid int8 NOT NULL,
  tid int8 NOT NULL,
  cell_data timestamptz NOT NULL
);

CREATE TABLE k_tab_cell_date_hist (
  table_otid int8 NOT NULL,
  col_otid int8 NOT NULL,
  row_otid int8 NOT NULL,
  tid int8 NOT NULL,
  cell_data timestamptz NOT NULL
);

CREATE INDEX k_tab_cell_date_hist_col_otid ON k_tab_cell_date_hist USING btree (col_otid);

CREATE INDEX k_tab_cell_date_hist_row_otid ON k_tab_cell_date_hist USING btree (row_otid);

CREATE INDEX k_tab_cell_date_hist_table_otid ON k_tab_cell_date_hist USING btree (table_otid);

CREATE UNIQUE INDEX k_tab_cell_date_hist_ukey_tid ON k_tab_cell_date_hist USING btree (tid);

CREATE UNIQUE INDEX k_tab_cell_date_pkey1 ON k_tab_cell_date USING btree (table_otid, col_otid, row_otid);

CREATE UNIQUE INDEX k_tab_cell_date_ukey_tid ON k_tab_cell_date USING btree (tid);

CREATE TABLE k_tab_cell_decimal (
  table_otid int8 NOT NULL,
  col_otid int8 NOT NULL,
  row_otid int8 NOT NULL,
  tid int8 NOT NULL,
  cell_data text NOT NULL
);

CREATE TABLE k_tab_cell_decimal_hist (
  table_otid int8 NOT NULL,
  col_otid int8 NOT NULL,
  row_otid int8 NOT NULL,
  tid int8 NOT NULL,
  cell_data text NOT NULL
);

CREATE INDEX k_tab_cell_decimal_hist_col_otid ON k_tab_cell_decimal_hist USING btree (col_otid);

CREATE INDEX k_tab_cell_decimal_hist_row_otid ON k_tab_cell_decimal_hist USING btree (row_otid);

CREATE INDEX k_tab_cell_decimal_hist_table_otid ON k_tab_cell_decimal_hist USING btree (table_otid);

CREATE UNIQUE INDEX k_tab_cell_decimal_hist_ukey_tid ON k_tab_cell_decimal_hist USING btree (tid);

CREATE UNIQUE INDEX k_tab_cell_decimal_pkey1 ON k_tab_cell_decimal USING btree (table_otid, col_otid, row_otid);

CREATE UNIQUE INDEX k_tab_cell_decimal_ukey_tid ON k_tab_cell_decimal USING btree (tid);

CREATE TABLE k_tab_cell_text (
  table_otid int8 NOT NULL,
  col_otid int8 NOT NULL,
  row_otid int8 NOT NULL,
  tid int8 NOT NULL,
  cell_data text NOT NULL
);

CREATE TABLE k_tab_cell_text_hist (
  table_otid int8 NOT NULL,
  col_otid int8 NOT NULL,
  row_otid int8 NOT NULL,
  tid int8 NOT NULL,
  cell_data text NOT NULL
);

CREATE INDEX k_tab_cell_text_hist_col_otid ON k_tab_cell_text_hist USING btree (col_otid);

CREATE INDEX k_tab_cell_text_hist_row_otid ON k_tab_cell_text_hist USING btree (row_otid);

CREATE INDEX k_tab_cell_text_hist_table_otid ON k_tab_cell_text_hist USING btree (table_otid);

CREATE UNIQUE INDEX k_tab_cell_text_hist_ukey_tid ON k_tab_cell_text_hist USING btree (tid);

CREATE UNIQUE INDEX k_tab_cell_text_pkey1 ON k_tab_cell_text USING btree (table_otid, col_otid, row_otid);

CREATE UNIQUE INDEX k_tab_cell_text_ukey_tid ON k_tab_cell_text USING btree (tid);

CREATE TABLE k_tab_meta (
  otid int8 NOT NULL,
  columns text NOT NULL,
  table_name varchar(300) NOT NULL,
  table_comment varchar(1000) NOT NULL,
  update_time timestamptz,
  real_table bool NOT NULL,
  tid int8 NOT NULL
);

CREATE TABLE k_tab_meta_hist (
  otid int8 NOT NULL,
  columns text NOT NULL,
  table_name varchar(300) NOT NULL,
  table_comment varchar(1000) NOT NULL,
  update_time timestamptz,
  real_table bool NOT NULL,
  tid int8 NOT NULL
);

CREATE INDEX k_tab_meta_hist_otid ON k_tab_meta_hist USING btree (otid);

CREATE UNIQUE INDEX k_tab_meta_hist_ukey_tid ON k_tab_meta_hist USING btree (tid);

CREATE UNIQUE INDEX k_tab_meta_pkey1 ON k_tab_meta USING btree (otid);

CREATE UNIQUE INDEX k_tab_meta_ukey_tid ON k_tab_meta USING btree (tid);

CREATE TABLE kkv (
  key varchar(500) NOT NULL,
  kind varchar(100) NOT NULL,
  kspace varchar(40) NOT NULL,
  tid int8 NOT NULL,
  archor bool NOT NULL,
  value text NOT NULL
);

CREATE TABLE kkv_hist (
  key varchar(500) NOT NULL,
  kind varchar(100) NOT NULL,
  kspace varchar(40) NOT NULL,
  tid int8 NOT NULL,
  archor bool NOT NULL,
  value text NOT NULL
);

CREATE INDEX kkv_hist_key ON kkv_hist USING btree (key);

CREATE INDEX kkv_hist_kind ON kkv_hist USING btree (kind);

CREATE INDEX kkv_hist_kspace ON kkv_hist USING btree (kspace);

CREATE UNIQUE INDEX kkv_hist_ukey_tid ON kkv_hist USING btree (tid);

CREATE UNIQUE INDEX kkv_pkey2 ON kkv USING btree (key, kind, kspace);

CREATE TABLE kkv_transient (
  key varchar(500) NOT NULL,
  value text NOT NULL,
  tid int8 NOT NULL
);

CREATE UNIQUE INDEX kkv_transient_pkey1 ON kkv_transient USING btree (key);

CREATE UNIQUE INDEX kkv_transient_ukey_tid ON kkv_transient USING btree (tid);

CREATE UNIQUE INDEX kkv_ukey_tid ON kkv USING btree (tid);

CREATE TABLE llm_chat_bot (
  otid int8 NOT NULL,
  name varchar(500) NOT NULL,
  body text NOT NULL,
  svg_logo text,
  update_time timestamptz,
  tid int8 NOT NULL
);

CREATE TABLE llm_chat_bot_hist (
  otid int8 NOT NULL,
  name varchar(500) NOT NULL,
  body text NOT NULL,
  svg_logo text,
  update_time timestamptz,
  tid int8 NOT NULL
);

CREATE INDEX llm_chat_bot_hist_otid ON llm_chat_bot_hist USING btree (otid);

CREATE UNIQUE INDEX llm_chat_bot_hist_ukey_tid ON llm_chat_bot_hist USING btree (tid);

CREATE UNIQUE INDEX llm_chat_bot_pkey1 ON llm_chat_bot USING btree (otid);

CREATE UNIQUE INDEX llm_chat_bot_ukey_tid ON llm_chat_bot USING btree (tid);

CREATE TABLE llm_chat_record (
  otid int8 NOT NULL,
  session_otid int8 NOT NULL,
  pre_record_otid int8,
  content text NOT NULL,
  reasoning_content text NOT NULL,
  role varchar(40) NOT NULL,
  role_id int8,
  tid int8 NOT NULL
);

CREATE TABLE llm_chat_record_hist (
  otid int8 NOT NULL,
  session_otid int8 NOT NULL,
  pre_record_otid int8,
  content text NOT NULL,
  reasoning_content text NOT NULL,
  role varchar(40) NOT NULL,
  role_id int8,
  tid int8 NOT NULL
);

CREATE INDEX llm_chat_record_hist_otid ON llm_chat_record_hist USING btree (otid);

CREATE UNIQUE INDEX llm_chat_record_hist_ukey_tid ON llm_chat_record_hist USING btree (tid);

CREATE UNIQUE INDEX llm_chat_record_pkey2 ON llm_chat_record USING btree (otid);

CREATE UNIQUE INDEX llm_chat_record_ukey_tid ON llm_chat_record USING btree (tid);

CREATE TABLE llm_chat_session (
  otid int8 NOT NULL,
  template_otid int8 NOT NULL,
  title varchar(500) NOT NULL,
  update_time timestamptz,
  tid int8 NOT NULL
);

CREATE TABLE llm_chat_session_hist (
  otid int8 NOT NULL,
  template_otid int8 NOT NULL,
  title varchar(500) NOT NULL,
  update_time timestamptz,
  tid int8 NOT NULL
);

CREATE INDEX llm_chat_session_hist_otid ON llm_chat_session_hist USING btree (otid);

CREATE UNIQUE INDEX llm_chat_session_hist_ukey_tid ON llm_chat_session_hist USING btree (tid);

CREATE UNIQUE INDEX llm_chat_session_pkey1 ON llm_chat_session USING btree (otid);

CREATE UNIQUE INDEX llm_chat_session_ukey_tid ON llm_chat_session USING btree (tid);

CREATE TABLE llm_chat_template (
  otid int8 NOT NULL,
  name varchar(200) NOT NULL,
  prompt text NOT NULL,
  svg_logo text,
  update_time timestamptz,
  tid int8 NOT NULL
);

CREATE TABLE llm_chat_template_hist (
  otid int8 NOT NULL,
  name varchar(200) NOT NULL,
  prompt text NOT NULL,
  svg_logo text,
  update_time timestamptz,
  tid int8 NOT NULL
);

CREATE INDEX llm_chat_template_hist_otid ON llm_chat_template_hist USING btree (otid);

CREATE UNIQUE INDEX llm_chat_template_hist_ukey_tid ON llm_chat_template_hist USING btree (tid);

CREATE UNIQUE INDEX llm_chat_template_pkey1 ON llm_chat_template USING btree (otid);

CREATE UNIQUE INDEX llm_chat_template_ukey_tid ON llm_chat_template USING btree (tid);

CREATE TABLE sync_log_transient (
  remote_id varchar(100) NOT NULL,
  table_name varchar(100) NOT NULL,
  end_sync_in int8 NOT NULL,
  start_tid_ex int8 NOT NULL,
  sync_finish_tid int8 NOT NULL
);