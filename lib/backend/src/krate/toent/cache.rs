use std::collections::BTreeMap;

use anyhow::Context;
use chin_sql::time_type::TID;
use chin_tools::{AResult, EResult};
use chrono::Utc;

use crate::{
    app::ShareAppState,
    krate::toent::{ToentSearchReq, mapper::ToentMapper, po::ToentInst},
};

#[derive(Debug)]
pub(crate) struct ToentCache {
    pub(crate) start_tid: TID,
    pub(crate) end_tid: TID,
    pub(crate) otid_inst_map: BTreeMap<TID, ToentInst>,
}

impl Default for ToentCache {
    fn default() -> Self {
        let now = Utc::now();
        let start_of_day = now
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .expect("Invalid time")
            .and_local_timezone(Utc)
            .unwrap();
        // 3. Get the End of the Day (23:59:59.999...)
        let end_of_day = now
            .date_naive()
            .and_hms_micro_opt(23, 59, 59, 999_999)
            .expect("Invalid time")
            .and_local_timezone(Utc)
            .unwrap();

        Self {
            otid_inst_map: BTreeMap::new(),
            start_tid: start_of_day.timestamp_micros().try_into().unwrap(),
            end_tid: end_of_day.timestamp_micros().try_into().unwrap(),
        }
    }
}

fn today_tid_window() -> AResult<(chrono::NaiveDate, TID, TID)> {
    let today = Utc::now().date_naive();
    let start = today
        .and_hms_opt(0, 0, 0)
        .context("unable to build today's start time")?;
    let end = (today + chrono::Days::new(1))
        .and_hms_opt(0, 0, 0)
        .context("unable to build tomorrow's start time")?;

    let start_tid = TID::try_from(start.and_utc().timestamp_micros())?;
    let end_tid = TID::try_from(end.and_utc().timestamp_micros())?;
    Ok((today, start_tid, end_tid))
}

fn toent_today_search_req() -> AResult<ToentSearchReq> {
    let (_, start_tid, end_tid) = today_tid_window()?;
    Ok(ToentSearchReq {
        start_tid,
        end_tid,
        include_completed: true,
        include_uncompleted: true,
        include_no_time_todo: true,
        query: None,
        tags: None,
        start_index: 0,
        page_size: 500,
        chnot_otids: None,
    })
}

impl ToentCache {
    pub(crate) async fn refresh(app: &ShareAppState) -> EResult {
        let today_req = toent_today_search_req()?;
        let mut start_index = 0usize;
        let mut map: BTreeMap<TID, ToentInst> = BTreeMap::new();

        loop {
            let rsp = app
                .mapper
                .toent_search(
                    ToentSearchReq {
                        start_index,
                        ..today_req.clone()
                    },
                    vec![],
                )
                .await?;
            for item in rsp.items {
                for inst in item.inst {
                    map.entry(inst.otid).or_insert(inst);
                }
            }

            if !rsp.has_next {
                break;
            }
            if rsp.next_start <= start_index {
                break;
            }
            start_index = rsp.next_start;
        }

        *app.toent_cache.write().await = ToentCache {
            otid_inst_map: map,
            start_tid: today_req.start_tid,
            end_tid: today_req.end_tid,
        };

        Ok(())
    }

    pub(crate) async fn refresh_chnots(app: &ShareAppState, otids: Vec<TID>) -> EResult {
        let mut today_req = toent_today_search_req()?;
        today_req.chnot_otids.replace(otids);
        let mut start_index = 0usize;

        loop {
            let rsp = app
                .mapper
                .toent_search(
                    ToentSearchReq {
                        start_index,
                        ..today_req.clone()
                    },
                    vec![],
                )
                .await?;
            let mut toent_cache = app.toent_cache.write().await;
            for item in rsp.items {
                for inst in item.inst {
                    toent_cache.overwrite_toent_inst(inst);
                }
            }

            if !rsp.has_next {
                break;
            }
            if rsp.next_start <= start_index {
                break;
            }
            start_index = rsp.next_start;
        }

        Ok(())
    }

    pub(crate) fn overwrite_toent_inst(&mut self, toent_inst: ToentInst) {
        let can_insert = toent_inst
            .alert_tid
            .is_some_and(|e| e < self.end_tid && e >= self.start_tid)
            || toent_inst
                .start_tid
                .is_some_and(|e| e < self.end_tid && e >= self.start_tid);
        if can_insert {
            self.otid_inst_map.insert(toent_inst.otid, toent_inst);
        }
    }

    pub(crate) fn remove_chnot(&mut self, chnot_otid: TID) -> EResult {
        self.otid_inst_map
            .retain(|_k, v| v.chnot_otid != chnot_otid);
        Ok(())
    }
}
