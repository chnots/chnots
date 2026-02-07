alter table chnot_metadata
rename to chnot_metadata_bak;

alter table chnot_metadata_hist
rename to chnot_metadata_hist_bak;

alter table chnot_record
rename to chnot_record_bak;

alter table chnot_record_hist
rename to chnot_record_hist_bak;

alter table chnot_tag
rename to chnot_tag_bak;

alter table chnot_tag_hist
rename to chnot_tag_hist_bak;

alter table k_file_meta
rename to k_file_meta_bak;

alter table k_file_meta_hist
rename to k_file_meta_hist_bak;

alter table llm_chat_record
rename to llm_chat_record_bak;

alter table llm_chat_record_hist
rename to llm_chat_record_hist_bak;

alter table chnot_kind_rel
rename to chnot_kind_rel_bak;

alter table chnot_kind_rel_hist
rename to chnot_kind_rel_hist_bak;

-- graph part
create table
  graph_meta_hist (
    otid INT8 not null,
    archor BOOL not null,
    kind Varchar(16) not null,
    content TEXT not null,
    tid INT8 not null
  );

drop index if exists graph_meta_hist_ukey_tid;

create unique index graph_meta_hist_ukey_tid on graph_meta_hist (tid);

create index graph_meta_hist_otid on graph_meta_hist (otid);

create table
  graph_meta (
    otid INT8 not null,
    archor BOOL not null,
    kind Varchar(16) not null,
    content TEXT not null,
    tid INT8 not null,
    primary key (otid)
  );

drop index if exists graph_meta_ukey_tid;

create unique index graph_meta_ukey_tid on graph_meta (tid);

CREATE TABLE
  public.graph_data (
    sid character varying(100) NOT NULL,
    tid bigint NOT NULL,
    content text NOT NULL
  );

ALTER TABLE ONLY public.graph_data ADD CONSTRAINT graph_data_pkey PRIMARY KEY (sid);

CREATE UNIQUE INDEX graph_data_ukey_tid ON public.graph_data USING btree (tid);

create table
  chnot_thread_order_hist (
    otid INT8 not null,
    thread_otid INT8 not null,
    korder INT8 not null,
    tid INT8 not null
  );

drop index if exists chnot_thread_order_hist_ukey_tid;

create unique index chnot_thread_order_hist_ukey_tid on chnot_thread_order_hist (tid);

create index chnot_thread_order_hist_otid on chnot_thread_order_hist (otid);

create index chnot_thread_order_hist_thread_otid on chnot_thread_order_hist (thread_otid);

create table
  chnot_thread_order (
    otid INT8 not null,
    thread_otid INT8 not null,
    korder INT8 not null,
    tid INT8 not null,
    primary key (otid, thread_otid)
  );

create unique index chnot_thread_order_ukey_tid on chnot_thread_order (tid);

create table
  chnot_thread_meta_hist (
    otid INT8 not null,
    kspace Varchar(40) not null,
    pin_tid INT8,
    archive_tid INT8,
    tid INT8 not null
  );

create unique index chnot_thread_meta_hist_ukey_tid on chnot_thread_meta_hist (tid);

create index chnot_thread_meta_hist_otid on chnot_thread_meta_hist (otid);

create table
  chnot_thread_meta (
    otid INT8 not null,
    kspace Varchar(40) not null,
    pin_tid INT8,
    archive_tid INT8,
    tid INT8 not null,
    primary key (otid)
  );

create unique index chnot_thread_meta_ukey_tid on chnot_thread_meta (tid);

create table
  chnot_meta_hist (
    otid INT8 not null,
    kind Varchar(40) not null,
    kspace Varchar(40) not null,
    archive_tid INT8,
    pin_tid INT8,
    tid INT8 not null
  );

create unique index chnot_meta_hist_ukey_tid on chnot_meta_hist (tid);

create index chnot_meta_hist_otid on chnot_meta_hist (otid);

create table
  chnot_meta (
    otid INT8 not null,
    kind Varchar(40) not null,
    kspace Varchar(40) not null,
    archive_tid INT8,
    pin_tid INT8,
    tid INT8 not null,
    primary key (otid)
  );

create unique index chnot_meta_ukey_tid on chnot_meta (tid);

create table
  llm_chat_record_hist (
    otid INT8 not null,
    session_otid INT8 not null,
    pre_record_otid INT8,
    content TEXT not null,
    role Varchar(40) not null,
    role_id INT8,
    tid INT8 not null
  );

drop index if exists llm_chat_record_hist_ukey_tid;

create unique index llm_chat_record_hist_ukey_tid on llm_chat_record_hist (tid);

drop index if exists llm_chat_record_hist_otid;

create index llm_chat_record_hist_otid on llm_chat_record_hist (otid);

create table
  llm_chat_record (
    otid INT8 not null,
    session_otid INT8 not null,
    pre_record_otid INT8,
    content TEXT not null,
    role Varchar(40) not null,
    role_id INT8,
    tid INT8 not null,
    primary key (otid)
  );

drop index if exists llm_chat_record_ukey_tid;

create unique index llm_chat_record_ukey_tid on llm_chat_record (tid);

create table
  mdwt_record_hist (
    otid INT8 not null,
    tid INT8 not null,
    todo_event Varchar(20),
    content TEXT not null,
    archor BOOL not null
  );

drop index if exists mdwt_record_hist_ukey_tid;

create unique index mdwt_record_hist_ukey_tid on mdwt_record_hist (tid);

create index mdwt_record_hist_otid on mdwt_record_hist (otid);

create table
  mdwt_record (
    otid INT8 not null,
    tid INT8 not null,
    todo_event Varchar(20),
    content TEXT not null,
    archor BOOL not null,
    primary key (otid)
  );

drop index if exists mdwt_record_ukey_tid;

create unique index mdwt_record_ukey_tid on mdwt_record (tid);

create table
  mdwt_tag_hist (
    tag Varchar(800) not null,
    mdwt_otid INT8 not null,
    kspace Varchar(40) not null,
    tid INT8 not null
  );

drop index if exists mdwt_tag_hist_ukey_tid;

create unique index mdwt_tag_hist_ukey_tid on mdwt_tag_hist (tid);

create index mdwt_tag_hist_tag on mdwt_tag_hist (tag);

create index mdwt_tag_hist_mdwt_otid on mdwt_tag_hist (mdwt_otid);

create table
  mdwt_tag (
    tag Varchar(800) not null,
    mdwt_otid INT8 not null,
    kspace Varchar(40) not null,
    tid INT8 not null,
    primary key (tag, mdwt_otid)
  );

create unique index mdwt_tag_ukey_tid on mdwt_tag (tid);

create table
  k_file_meta_hist (
    otid INT8 not null,
    id Varchar(100) not null,
    inline BOOL not null,
    archor BOOL not null,
    filename Varchar(1024) not null,
    content_type Varchar(200) not null,
    last_modified INT8 not null,
    sid Varchar(100) not null,
    filesize INT8 not null,
    tid INT8 not null,
    binaryp BOOL not null
  );

drop index if exists k_file_meta_hist_ukey_tid;

create unique index k_file_meta_hist_ukey_tid on k_file_meta_hist (tid);

drop index if exists k_file_meta_hist_ukey_id;

create index k_file_meta_hist_ukey_id on k_file_meta_hist (id);

create index k_file_meta_hist_otid on k_file_meta_hist (otid);

create table
  k_file_meta (
    otid INT8 not null,
    id Varchar(100) not null,
    inline BOOL not null,
    archor BOOL not null,
    filename Varchar(1024) not null,
    content_type Varchar(200) not null,
    last_modified INT8 not null,
    sid Varchar(100) not null,
    filesize INT8 not null,
    tid INT8 not null,
    binaryp BOOL not null,
    primary key (otid)
  );

drop index if exists k_file_meta_ukey_id;

create unique index k_file_meta_ukey_id on k_file_meta (id);

drop index if exists k_file_meta_ukey_tid;

create unique index k_file_meta_ukey_tid on k_file_meta (tid);

insert into
  chnot_meta (otid, kspace, kind, tid, archive_tid)
select
  otid,
  kspace,
  kind,
  tid,
  date_part ('epoch', archive_time) * 1000000
from
  chnot_metadata_bak cm
where
  cm.kind = 'mdwt';

insert into
  chnot_meta (otid, kspace, kind, tid, archive_tid)
select
  cast(ckr.kind_id as bigint) as otid,
  kspace,
  kind,
  cm.tid,
  date_part ('epoch', archive_time) * 1000000
from
  chnot_metadata_bak cm
  left join chnot_kind_rel_bak ckr on cm.otid = ckr.meta_otid
where
  (
    cm.kind = 'ktabv1'
    or cm.kind = 'llm_chat'
  )
  and ckr.kind_id is not null;

insert into
  chnot_meta (otid, kspace, kind, tid, archive_tid)
select
  otid,
  kspace,
  kind,
  tid,
  date_part ('epoch', archive_time) * 1000000
from
  chnot_metadata_bak cm
where
  cm.kind = 'resov1'
  or cm.kind = 'exdrv1';

insert into
  mdwt_record (otid, tid, todo_event, content, archor)
select
  cr.meta_otid,
  cr.tid,
  cr.todo_event,
  cr.content,
  cr.archor
from
  chnot_record_bak cr
  left join chnot_metadata_bak cm on cr.meta_otid = cm.otid
where
  cm.kind = 'mdwt';

insert into
  mdwt_record (otid, tid, todo_event, content, archor)
select
  cr.meta_otid,
  cr.tid,
  cr.todo_event,
  cr.content,
  cr.archor
from
  chnot_record_bak cr
  left join chnot_metadata_bak cm on cr.meta_otid = cm.otid
where
  cm.kind = 'resov1'
  or cm.kind = 'exdrv1';

insert into
  mdwt_record (otid, tid, todo_event, content, archor)
select
  cast(ckr.kind_id as bigint) as otid,
  cr.tid,
  cr.todo_event,
  cr.content,
  cr.archor
from
  chnot_record_bak cr
  left join chnot_metadata_bak cm on cr.meta_otid = cm.otid
  left join chnot_kind_rel_bak ckr on cm.otid = ckr.meta_otid
where
  (
    cm.kind = 'ktabv1'
    or cm.kind = 'llm_chat'
  )
  and kind_id is not null;

insert into
  mdwt_tag (tag, mdwt_otid, kspace, tid)
select
  tag,
  meta_otid,
  kspace,
  tid
from
  chnot_tag_bak;

insert into
  k_file_meta (
    otid,
    inline,
    archor,
    content_type,
    filename,
    filesize,
    id,
    last_modified,
    sid,
    tid,
    binaryp
  )
select distinct
  case
    when (cm.meta_otid is not null) then cm.meta_otid
    else fm.tid
  end as otid,
  fm.inline,
  fm.archor,
  fm.content_type,
  fm.filename,
  fm.filesize,
  fm.id,
  fm.last_modified,
  fm.sid,
  fm.tid,
  true
from
  k_file_meta_bak fm
  left join chnot_kind_rel_bak cm on fm.id = cm.kind_id;

insert into
  chnot_meta_hist (otid, kspace, kind, tid, archive_tid)
select
  otid,
  kspace,
  kind,
  tid,
  date_part ('epoch', archive_time) * 1000000
from
  chnot_metadata_hist_bak cm
where
  cm.kind = 'mdwt';

insert into
  chnot_meta_hist (otid, kspace, kind, tid, archive_tid)
select
  cast(ckr.kind_id as bigint) as otid,
  kspace,
  kind,
  cm.tid,
  date_part ('epoch', archive_time) * 1000000
from
  chnot_metadata_hist_bak cm
  left join chnot_kind_rel_hist_bak ckr on cm.otid = ckr.meta_otid
where
  (
    cm.kind = 'ktabv1'
    or cm.kind = 'llm_chat'
  )
  and ckr.kind_id is not null;

insert into
  chnot_meta_hist (otid, kspace, kind, tid, archive_tid)
select
  otid,
  kspace,
  kind,
  tid,
  date_part ('epoch', archive_time) * 1000000
from
  chnot_metadata_hist_bak cm
where
  cm.kind = 'resov1'
  or cm.kind = 'exdrv1';

insert into
  mdwt_record_hist (otid, tid, todo_event, content, archor)
select distinct
  cr.meta_otid,
  cr.tid,
  cr.todo_event,
  cr.content,
  cr.archor
from
  chnot_record_hist_bak cr
  left join chnot_metadata_hist_bak cm on cr.meta_otid = cm.otid
where
  cm.kind = 'mdwt';

insert into
  mdwt_record_hist (otid, tid, todo_event, content, archor)
select distinct
  cr.meta_otid,
  cr.tid,
  cr.todo_event,
  cr.content,
  cr.archor
from
  chnot_record_hist_bak cr
  left join chnot_metadata_hist_bak cm on cr.meta_otid = cm.otid
where
  cm.kind = 'resov1'
  or cm.kind = 'exdrv1';

insert into
  mdwt_record_hist (otid, tid, todo_event, content, archor)
select distinct
  cast(ckr.kind_id as bigint) as otid,
  cr.tid,
  cr.todo_event,
  cr.content,
  cr.archor
from
  chnot_record_hist_bak cr
  left join chnot_metadata_hist_bak cm on cr.meta_otid = cm.otid
  left join chnot_kind_rel_hist_bak ckr on cm.otid = ckr.meta_otid
where
  (
    cm.kind = 'ktabv1'
    or cm.kind = 'llm_chat'
  )
  and kind_id is not null;

insert into
  mdwt_tag_hist (tag, mdwt_otid, kspace, tid)
select
  tag,
  meta_otid,
  kspace,
  tid
from
  chnot_tag_hist_bak;

insert into
  k_file_meta_hist (
    otid,
    inline,
    archor,
    content_type,
    filename,
    filesize,
    id,
    last_modified,
    sid,
    tid,
    binaryp
  )
select distinct
  case
    when (cm.meta_otid is not null) then cm.meta_otid
    else fm.tid
  end as otid,
  fm.inline,
  fm.archor,
  fm.content_type,
  fm.filename,
  fm.filesize,
  fm.id,
  fm.last_modified,
  fm.sid,
  fm.tid,
  true
from
  k_file_meta_hist_bak fm
  left join chnot_kind_rel_hist_bak cm on fm.id = cm.kind_id;