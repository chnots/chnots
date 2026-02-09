CREATE TABLE
    inline_k_file (
        sid TEXT not null,
        tid INTEGER not null,
        content TEXT not null,
        primary key (sid)
    );

CREATE TABLE
    chnot_kind_rel (
        meta_otid INTEGER not null,
        kind_id TEXT not null,
        tid INTEGER not null,
        primary key (meta_otid)
    );

CREATE TABLE
    chnot_kind_rel_hist (
        meta_otid INTEGER not null,
        kind_id TEXT not null,
        tid INTEGER not null
    );

CREATE TABLE
    chnot_metadata (
        otid INTEGER not null,
        kspace TEXT not null,
        kind TEXT not null,
        pin_time INTEGER,
        archive_time INTEGER,
        tid INTEGER not null,
        primary key (otid)
    );

CREATE TABLE
    chnot_metadata_hist (
        otid INTEGER not null,
        kspace TEXT not null,
        kind TEXT not null,
        pin_time INTEGER,
        archive_time INTEGER,
        tid INTEGER not null
    );

CREATE TABLE
    chnot_record (
        meta_otid INTEGER not null,
        tid INTEGER not null,
        todo_event TEXT,
        content TEXT not null,
        archor INTEGER not null,
        primary key (meta_otid)
    );

CREATE TABLE
    chnot_record_hist (
        meta_otid INTEGER not null,
        tid INTEGER not null,
        todo_event TEXT,
        content TEXT not null,
        archor INTEGER not null
    );

CREATE TABLE
    chnot_tag (
        tag TEXT not null,
        meta_otid INTEGER not null,
        kspace TEXT not null,
        tid INTEGER not null,
        primary key (tag, meta_otid)
    );

CREATE TABLE
    chnot_tag_hist (
        tag TEXT not null,
        meta_otid INTEGER not null,
        kspace TEXT not null,
        tid INTEGER not null
    );

CREATE TABLE
    k_file_meta (
        id TEXT not null,
        inline INTEGER not null,
        archor INTEGER not null,
        tid INTEGER not null,
        filename TEXT not null,
        content_type TEXT not null,
        last_modified INTEGER not null,
        sid TEXT not null,
        filesize INTEGER not null,
        primary key (id)
    );

CREATE TABLE
    k_file_meta_hist (
        id TEXT not null,
        inline INTEGER not null,
        archor INTEGER not null,
        tid INTEGER not null,
        filename TEXT not null,
        content_type TEXT not null,
        last_modified INTEGER not null,
        sid TEXT not null,
        filesize INTEGER not null
    );

CREATE TABLE
    k_space (
        name TEXT not null,
        color TEXT not null,
        managers TEXT not null,
        tid INTEGER not null,
        public_access INTEGER not null,
        primary key (name)
    );

CREATE TABLE
    k_space_hist (
        name TEXT not null,
        color TEXT not null,
        managers TEXT not null,
        tid INTEGER not null,
        public_access INTEGER not null
    );

CREATE TABLE
    k_tab_cell_date (
        table_otid INTEGER not null,
        col_otid INTEGER not null,
        row_otid INTEGER not null,
        tid INTEGER not null,
        cell_data INTEGER not null,
        primary key (table_otid, col_otid, row_otid)
    );

CREATE TABLE
    k_tab_cell_date_hist (
        table_otid INTEGER not null,
        col_otid INTEGER not null,
        row_otid INTEGER not null,
        tid INTEGER not null,
        cell_data INTEGER not null
    );

CREATE TABLE
    k_tab_cell_decimal (
        table_otid INTEGER not null,
        col_otid INTEGER not null,
        row_otid INTEGER not null,
        tid INTEGER not null,
        cell_data TEXT not null,
        primary key (table_otid, col_otid, row_otid)
    );

CREATE TABLE
    k_tab_cell_decimal_hist (
        table_otid INTEGER not null,
        col_otid INTEGER not null,
        row_otid INTEGER not null,
        tid INTEGER not null,
        cell_data TEXT not null
    );

CREATE TABLE
    k_tab_cell_text (
        table_otid INTEGER not null,
        col_otid INTEGER not null,
        row_otid INTEGER not null,
        tid INTEGER not null,
        cell_data TEXT not null,
        primary key (table_otid, col_otid, row_otid)
    );

CREATE TABLE
    k_tab_cell_text_hist (
        table_otid INTEGER not null,
        col_otid INTEGER not null,
        row_otid INTEGER not null,
        tid INTEGER not null,
        cell_data TEXT not null
    );

CREATE TABLE
    k_tab_meta (
        otid INTEGER not null,
        columns TEXT not null,
        table_name TEXT not null,
        table_comment TEXT not null,
        update_time INTEGER,
        real_table INTEGER not null,
        tid INTEGER not null,
        primary key (otid)
    );

CREATE TABLE
    k_tab_meta_hist (
        otid INTEGER not null,
        columns TEXT not null,
        table_name TEXT not null,
        table_comment TEXT not null,
        update_time INTEGER,
        real_table INTEGER not null,
        tid INTEGER not null
    );

CREATE TABLE
    kkv (
        key TEXT not null,
        kind TEXT not null,
        kspace TEXT not null,
        tid INTEGER not null,
        archor INTEGER not null,
        value TEXT not null,
        primary key (key, kind, kspace)
    );

CREATE TABLE
    kkv_hist (
        key TEXT not null,
        kind TEXT not null,
        kspace TEXT not null,
        tid INTEGER not null,
        archor INTEGER not null,
        value TEXT not null
    );

CREATE TABLE
    kkv_transient (
        key TEXT not null,
        value TEXT not null,
        tid INTEGER not null,
        primary key (key)
    );

CREATE TABLE
    llm_chat_bot (
        otid INTEGER not null,
        name TEXT not null,
        body TEXT not null,
        svg_logo TEXT,
        update_time INTEGER,
        tid INTEGER not null,
        primary key (otid)
    );

CREATE TABLE
    llm_chat_bot_hist (
        otid INTEGER not null,
        name TEXT not null,
        body TEXT not null,
        svg_logo TEXT,
        update_time INTEGER,
        tid INTEGER not null
    );

CREATE TABLE
    llm_chat_record (
        otid INTEGER not null,
        session_otid INTEGER not null,
        pre_record_otid INTEGER,
        content TEXT not null,
        reasoning_content TEXT not null,
        role TEXT not null,
        role_id INTEGER,
        tid INTEGER not null,
        primary key (otid)
    );

CREATE TABLE
    llm_chat_record_hist (
        otid INTEGER not null,
        session_otid INTEGER not null,
        pre_record_otid INTEGER,
        content TEXT not null,
        reasoning_content TEXT not null,
        role TEXT not null,
        role_id INTEGER,
        tid INTEGER not null
    );

CREATE TABLE
    llm_chat_session (
        otid INTEGER not null,
        template_otid INTEGER not null,
        title TEXT not null,
        update_time INTEGER,
        tid INTEGER not null,
        primary key (otid)
    );

CREATE TABLE
    llm_chat_session_hist (
        otid INTEGER not null,
        template_otid INTEGER not null,
        title TEXT not null,
        update_time INTEGER,
        tid INTEGER not null
    );

CREATE TABLE
    llm_chat_template (
        otid INTEGER not null,
        name TEXT not null,
        prompt TEXT not null,
        svg_logo TEXT,
        update_time INTEGER,
        tid INTEGER not null,
        primary key (otid)
    );

CREATE TABLE
    llm_chat_template_hist (
        otid INTEGER not null,
        name TEXT not null,
        prompt TEXT not null,
        svg_logo TEXT,
        update_time INTEGER,
        tid INTEGER not null
    );

CREATE TABLE
    sync_log_transient (
        remote_id TEXT not null,
        table_name TEXT not null,
        end_sync_in INTEGER not null,
        start_tid_ex INTEGER not null,
        sync_finish_tid INTEGER not null
    );

CREATE INDEX chnot_kind_rel_hist_meta_otid on chnot_kind_rel_hist (meta_otid);

CREATE UNIQUE INDEX chnot_kind_rel_hist_ukey_tid on chnot_kind_rel_hist (tid);

CREATE UNIQUE INDEX chnot_kind_rel_ukey_tid on chnot_kind_rel (tid);

CREATE INDEX chnot_metadata_hist_otid on chnot_metadata_hist (otid);

CREATE UNIQUE INDEX chnot_metadata_hist_ukey_tid on chnot_metadata_hist (tid);

CREATE UNIQUE INDEX chnot_metadata_ukey_tid on chnot_metadata (tid);

CREATE INDEX chnot_record_hist_meta_otid on chnot_record_hist (meta_otid);

CREATE UNIQUE INDEX chnot_record_hist_ukey_tid on chnot_record_hist (tid);

CREATE UNIQUE INDEX chnot_record_ukey_tid on chnot_record (tid);

CREATE INDEX chnot_tag_hist_meta_otid on chnot_tag_hist (meta_otid);

CREATE INDEX chnot_tag_hist_tag on chnot_tag_hist (tag);

CREATE UNIQUE INDEX chnot_tag_hist_ukey_tid on chnot_tag_hist (tid);

CREATE UNIQUE INDEX chnot_tag_ukey_tid on chnot_tag (tid);

CREATE UNIQUE INDEX inline_k_file_ukey_tid on inline_k_file (tid);

CREATE INDEX k_file_meta_hist_id on k_file_meta_hist (id);

CREATE UNIQUE INDEX k_file_meta_hist_ukey_tid on k_file_meta_hist (tid);

CREATE UNIQUE INDEX k_file_meta_ukey_tid on k_file_meta (tid);

CREATE INDEX k_space_hist_name on k_space_hist (name);

CREATE UNIQUE INDEX k_space_hist_ukey_tid on k_space_hist (tid);

CREATE UNIQUE INDEX k_space_ukey_tid on k_space (tid);

CREATE INDEX k_tab_cell_date_hist_col_otid on k_tab_cell_date_hist (col_otid);

CREATE INDEX k_tab_cell_date_hist_row_otid on k_tab_cell_date_hist (row_otid);

CREATE INDEX k_tab_cell_date_hist_table_otid on k_tab_cell_date_hist (table_otid);

CREATE UNIQUE INDEX k_tab_cell_date_hist_ukey_tid on k_tab_cell_date_hist (tid);

CREATE UNIQUE INDEX k_tab_cell_date_ukey_tid on k_tab_cell_date (tid);

CREATE INDEX k_tab_cell_decimal_hist_col_otid on k_tab_cell_decimal_hist (col_otid);

CREATE INDEX k_tab_cell_decimal_hist_row_otid on k_tab_cell_decimal_hist (row_otid);

CREATE INDEX k_tab_cell_decimal_hist_table_otid on k_tab_cell_decimal_hist (table_otid);

CREATE UNIQUE INDEX k_tab_cell_decimal_hist_ukey_tid on k_tab_cell_decimal_hist (tid);

CREATE UNIQUE INDEX k_tab_cell_decimal_ukey_tid on k_tab_cell_decimal (tid);

CREATE INDEX k_tab_cell_text_hist_col_otid on k_tab_cell_text_hist (col_otid);

CREATE INDEX k_tab_cell_text_hist_row_otid on k_tab_cell_text_hist (row_otid);

CREATE INDEX k_tab_cell_text_hist_table_otid on k_tab_cell_text_hist (table_otid);

CREATE UNIQUE INDEX k_tab_cell_text_hist_ukey_tid on k_tab_cell_text_hist (tid);

CREATE UNIQUE INDEX k_tab_cell_text_ukey_tid on k_tab_cell_text (tid);

CREATE INDEX k_tab_meta_hist_otid on k_tab_meta_hist (otid);

CREATE UNIQUE INDEX k_tab_meta_hist_ukey_tid on k_tab_meta_hist (tid);

CREATE UNIQUE INDEX k_tab_meta_ukey_tid on k_tab_meta (tid);

CREATE INDEX kkv_hist_key on kkv_hist (key);

CREATE INDEX kkv_hist_kind on kkv_hist (kind);

CREATE INDEX kkv_hist_kspace on kkv_hist (kspace);

CREATE UNIQUE INDEX kkv_hist_ukey_tid on kkv_hist (tid);

CREATE UNIQUE INDEX kkv_transient_ukey_tid on kkv_transient (tid);

CREATE UNIQUE INDEX kkv_ukey_tid on kkv (tid);

CREATE INDEX llm_chat_bot_hist_otid on llm_chat_bot_hist (otid);

CREATE UNIQUE INDEX llm_chat_bot_hist_ukey_tid on llm_chat_bot_hist (tid);

CREATE UNIQUE INDEX llm_chat_bot_ukey_tid on llm_chat_bot (tid);

CREATE INDEX llm_chat_record_hist_otid on llm_chat_record_hist (otid);

CREATE UNIQUE INDEX llm_chat_record_hist_ukey_tid on llm_chat_record_hist (tid);

CREATE UNIQUE INDEX llm_chat_record_ukey_tid on llm_chat_record (tid);

CREATE INDEX llm_chat_session_hist_otid on llm_chat_session_hist (otid);

CREATE UNIQUE INDEX llm_chat_session_hist_ukey_tid on llm_chat_session_hist (tid);

CREATE UNIQUE INDEX llm_chat_session_ukey_tid on llm_chat_session (tid);

CREATE INDEX llm_chat_template_hist_otid on llm_chat_template_hist (otid);

CREATE UNIQUE INDEX llm_chat_template_hist_ukey_tid on llm_chat_template_hist (tid);

CREATE UNIQUE INDEX llm_chat_template_ukey_tid on llm_chat_template (tid);