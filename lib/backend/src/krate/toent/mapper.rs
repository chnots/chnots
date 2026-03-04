use chin_sql::{Wheres, time_type::TID};
use chin_tools::EResult;
use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime, Utc};

use crate::{
    krate::toent::{
        EventDefi, EventDefiItem,
        po::{TodoInst, ToentEvent, ToentTodo},
    },
    mapper::db::KDbTx,
};

pub(crate) trait ToentMapper {
    async fn upsert_toent_todo(&self, todo: ToentTodo) -> EResult;
    async fn upsert_toent_event(&self, event: ToentEvent) -> EResult;
    async fn rebuild_todo_inst(
        &self,
        otid: TID,
        timezone: Option<String>,
        target_status: Option<crate::krate::toent::logic::todoevent::TodoStateEnum>,
        events: Vec<EventDefiItem>,
    ) -> EResult;
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

    async fn rebuild_todo_inst(
        &self,
        otid: TID,
        timezone: Option<String>,
        target_status: Option<crate::krate::toent::logic::todoevent::TodoStateEnum>,
        events: Vec<EventDefiItem>,
    ) -> EResult {
        let now = Utc::now();
        let window_end = now + Duration::days(30);
        self.omit_rows::<TodoInst>(Wheres::equal(TodoInst::OTID, otid))
            .await?;

        let mut insts = Vec::new();
        for event in events {
            let Some(parsed) = EventDefi::resolve_to_target(event) else {
                continue;
            };
            if parsed.utc < now || parsed.utc > window_end {
                continue;
            }
            let target_tid: TID = parsed.utc.timestamp_millis().try_into()?;
            insts.push(TodoInst {
                otid,
                timezone: parsed
                    .timezone
                    .or_else(|| timezone.clone())
                    .map(|e| e.try_into())
                    .transpose()?,
                naive_time: parsed.naive_time.try_into()?,
                target_status,
                note: None,
                alert_tid: None,
                target_tid,
                tid: TID::default(),
            });
        }

        if !insts.is_empty() {
            self.po_otid_insert(insts).await?;
        }

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
            let fixed = FixedOffset::parse_from_str(tz.as_str(), "%:z").ok()?;
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
