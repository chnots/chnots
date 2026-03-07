use chin_tools::{AResult, EResult};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

use crate::{
    MapperType, expand_mt_branch,
    krate::toent::{
        EventDefi, EventDefiItem, ToentInstCountReq, ToentInstCountRsp, ToentInstListReq,
        ToentInstListRsp, ToentSearchReq, ToentTodoStateCommitReq, ToentTodoStateCommitRsp,
        po::{ToentEvent, ToentTodo},
    },
    mapper::db::KDbTx,
    model::dto::KReq,
};

pub(crate) trait ToentMapper {
    async fn upsert_toent_todo(&self, todo: ToentTodo) -> EResult;
    async fn upsert_toent_event(&self, event: ToentEvent) -> EResult;
}

pub(crate) trait ToentReadMapper {
    async fn toent_inst_list(&self, req: KReq<ToentInstListReq>) -> AResult<ToentInstListRsp>;
    async fn toent_inst_count(&self, req: KReq<ToentInstCountReq>) -> AResult<ToentInstCountRsp>;
    async fn toent_search(&self, req: KReq<ToentSearchReq>) -> AResult<ToentInstListRsp>;
    async fn toent_todo_state_commit(
        &self,
        req: KReq<ToentTodoStateCommitReq>,
    ) -> AResult<ToentTodoStateCommitRsp>;
}

impl ToentReadMapper for MapperType {
    async fn toent_inst_list(&self, req: KReq<ToentInstListReq>) -> AResult<ToentInstListRsp> {
        expand_mt_branch!(self.toent_inst_list(req))
    }

    async fn toent_inst_count(&self, req: KReq<ToentInstCountReq>) -> AResult<ToentInstCountRsp> {
        expand_mt_branch!(self.toent_inst_count(req))
    }

    async fn toent_search(&self, req: KReq<ToentSearchReq>) -> AResult<ToentInstListRsp> {
        expand_mt_branch!(self.toent_search(req))
    }

    async fn toent_todo_state_commit(
        &self,
        req: KReq<ToentTodoStateCommitReq>,
    ) -> AResult<ToentTodoStateCommitRsp> {
        expand_mt_branch!(self.toent_todo_state_commit(req))
    }
}

impl ToentMapper for KDbTx<'_> {
    async fn upsert_toent_todo(&self, todo: ToentTodo) -> EResult {
        self.po_otid_insert([todo]).await?;
        Ok(())
    }

    async fn upsert_toent_event(&self, event: ToentEvent) -> EResult {
        self.po_otid_insert([event]).await?;
        Ok(())
    }
}

pub(crate) struct ResolvedTarget {
    pub(crate) utc: DateTime<Utc>,
    pub(crate) timezone: Option<String>,
    pub(crate) naive_time: String,
}

impl EventDefi {
    pub(crate) fn resolve_to_target(item: EventDefiItem) -> Option<ResolvedTarget> {
        let source = item.standard.as_ref().unwrap_or(&item.raw).trim();
        let naive = EventDefi::extract_naive_time(source)
            .or_else(|| EventDefi::extract_naive_time(item.raw.as_str()))?;

        let timezone = EventDefi::extract_timezone(source)
            .or_else(|| item.timezone.clone())
            .or_else(|| EventDefi::extract_timezone(item.raw.as_str()));

        let naive_dt = NaiveDateTime::parse_from_str(naive.as_str(), "%Y-%m-%d %H:%M:%S").ok()?;
        let utc = if let Some(tz) = timezone.clone() {
            let fixed = tz.parse::<FixedOffset>().ok()?;
            naive_dt.and_local_timezone(fixed).single()?.to_utc()
        } else {
            naive_dt.and_utc()
        };

        Some(ResolvedTarget {
            utc,
            timezone,
            naive_time: naive,
        })
    }

    fn extract_naive_time(source: &str) -> Option<String> {
        let caps = regex::Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})")
            .ok()?
            .captures(source)?;
        Some(caps.get(1)?.as_str().to_string())
    }

    fn extract_timezone(source: &str) -> Option<String> {
        let caps = regex::Regex::new(r"([+-]\d{1,2}:\d{2})")
            .ok()?
            .captures(source)?;
        let tz = caps.get(1)?.as_str();
        if tz.len() == 5 {
            Some(format!("{}0{}", &tz[..2], &tz[2..]))
        } else {
            Some(tz.to_string())
        }
    }
}
