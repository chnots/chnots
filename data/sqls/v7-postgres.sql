create table
    toent_inst (
        otid INT8 not null,
        todo_state Varchar(30),
        todo_priority INT8,
        alert_tid INT8,
        start_tid INT8,
        chnot_otid INT8 not null,
        end_tid INT8,
        finished_count INT8 not null,
        is_lunar BOOL not null,
        timezone INT8,
        closed BOOL,
        tid INT8 not null,
        note TEXT,
        primary key (otid, start_tid)
    );

create unique index toent_inst_ukey_tid on toent_inst (tid);

create table
    toent_inst_hist (
        otid INT8 not null,
        todo_state Varchar(30),
        todo_priority INT8,
        alert_tid INT8,
        start_tid INT8,
        chnot_otid INT8 not null,
        end_tid INT8,
        finished_count INT8 not null,
        is_lunar BOOL not null,
        timezone INT8,
        closed BOOL,
        tid INT8 not null,
        note TEXT
    );

create unique index toent_inst_hist_ukey_tid on toent_inst_hist (tid);

create index toent_inst_hist_otid on toent_inst_hist (otid);

create index toent_inst_hist_start_tid on toent_inst_hist (start_tid);

create table
    toent_defi (
        otid INT8 not null,
        event_defi TEXT,
        todo_state Varchar(30),
        todo_priority INT8,
        start_tid INT8,
        start_timezone INT8,
        end_tid INT8,
        end_timezone INT8,
        total_count INT8,
        tid INT8 not null,
        primary key (otid)
    );

create unique index toent_defi_ukey_tid on toent_defi (tid);

create table
    toent_defi_hist (
        otid INT8 not null,
        event_defi TEXT,
        todo_state Varchar(30),
        todo_priority INT8,
        start_tid INT8,
        start_timezone INT8,
        end_tid INT8,
        end_timezone INT8,
        total_count INT8,
        tid INT8 not null
    );

create unique index toent_defi_hist_ukey_tid on toent_defi_hist (tid);

create index toent_defi_hist_otid on toent_defi_hist (otid);
