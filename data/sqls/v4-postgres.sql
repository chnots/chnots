insert into
    chnot_meta (otid, kspace, kind, tid, archive_tid)
select
    otid,
    kspace,
    'threadv1' as kind,
    tid,
    archive_tid
from
    chnot_thread_meta;

insert into
    chnot_meta_hist (otid, kspace, kind, tid, archive_tid)
select
    otid,
    kspace,
    'threadv1' as kind,
    tid,
    archive_tid
from
    chnot_thread_meta_hist;

drop table chnot_thread_meta_hist;

drop table chnot_thread_meta;