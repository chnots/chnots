use anyhow::Context;
use axum::{Json, Router, extract::State, http::HeaderMap, routing::post};
use chin_sql::{str_type::Varchar, time_type::TID};
use chin_tools::AResult;
use lazy_regex::Lazy;
use regex::Regex;

use crate::{
    app::ShareAppState,
    controller::KResponse,
    krate::{
        mdwt::{MdwtCommitReq, MdwtCommitReqData, MdwtRecordsReq, mapper::MdwtMapper},
        toent::{
            cache::ToentCache,
            logic::{timeevent::TimeEvent, todoevent::TodoEvent},
            po::ToentInst,
            todoevent::TodoStateEnum,
        },
    },
    model::dto::{KReq, kreq, read_mkspace_from_header},
};

use super::*;
use crate::krate::toent::mapper::ToentMapper;

async fn toent_time_event_guess(
    Json(req): Json<ToentGuessReq>,
) -> KResponse<ToentGuessRsp<TimeEvent>> {
    let words: Words<'_> = req.input.as_str().into();
    let rest: Option<Vec<GuessElem<TimeEvent>>> = TimeEvent::guess(&words);
    let rsp = ToentGuessRsp {
        toents: rest
            .unwrap_or_default()
            .into_iter()
            .map(|e| e.timestamp)
            .collect(),
    };

    Ok(rsp).into()
}

async fn toent_todo_event_guess(
    Json(req): Json<ToentGuessReq>,
) -> KResponse<ToentGuessRsp<TodoEvent>> {
    let words: Words<'_> = req.input.as_str().into();
    let rest: Option<Vec<GuessElem<TodoEvent>>> = TodoEvent::guess(&words);
    let rsp = ToentGuessRsp {
        toents: rest
            .unwrap_or_default()
            .into_iter()
            .map(|e| e.timestamp)
            .collect(),
    };

    Ok(rsp).into()
}

async fn toent_inst_count(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ToentInstCountReq>,
) -> KResponse<ToentInstCountRsp> {
    state.toent_inst_count(kreq(headers, req)).await.into()
}

async fn toent_search(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ToentSearchReq>,
) -> KResponse<ToentSearchRsp> {
    let mkspaces = read_mkspace_from_header(&headers);
    state.toent_search(req, mkspaces).await.into()
}

async fn toent_inst_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ToentInstCommitReq>,
) -> KResponse<ToentInstCommitRsp> {
    state.toent_inst_commit(kreq(headers, req)).await.into()
}

static TODO_EVENT_REGEX: Lazy<Regex> = lazy_regex::lazy_regex!(r"\[([A-Z]+)(?: !([A-Z]))?\]");

fn rewrite_todo_state(content: &str, todo_state: TodoStateEnum) -> AResult<String> {
    let caps = TODO_EVENT_REGEX
        .captures(content)
        .context("unable to find todo marker from mdwt content")?;
    let priority = caps
        .get(2)
        .map(|p| format!(" !{}", p.as_str()))
        .unwrap_or_default();
    let replacement = format!("[{}{}]", todo_state.as_static_str(), priority);
    Ok(TODO_EVENT_REGEX
        .replacen(content, 1, replacement.as_str())
        .to_string())
}

impl ShareAppState {
    async fn toent_search(
        &self,
        req: ToentSearchReq,
        spaces: Vec<Varchar<40>>,
    ) -> AResult<ToentSearchRsp> {
        let rsp = self.mapper.toent_search(req.clone(), spaces).await?;

        Ok(rsp)
    }

    async fn toent_inst_commit(
        &self,
        req: KReq<ToentInstCommitReq>,
    ) -> AResult<ToentInstCommitRsp> {
        let rsp = self.mapper.toent_inst_commit(req.clone()).await?;
        let mut cache = self.toent_cache.write().await;
        for ele in rsp.updated_insts.clone() {
            cache.overwrite_toent_inst(ele);
        }
        Ok(rsp)
    }

    async fn toent_todo_state_commit(
        &self,
        req: KReq<ToentTodoStateCommitReq>,
    ) -> AResult<ToentTodoStateCommitRsp> {
        let mut rsp = self
            .mdwt_list(req.frame(MdwtRecordsReq {
                mdwt_otids: vec![req.otid],
            }))
            .await?;
        let mdwt = rsp.mdwt_map.remove(&req.otid).ok_or(anyhow::anyhow!(
            "unable to find the mdwt with otid: {}",
            req.otid
        ))?;
        let next_content = rewrite_todo_state(mdwt.content.as_str(), req.todo_state)?;
        let rsp = self
            .mdwt_commit(req.frame(MdwtCommitReq {
                mdwt: MdwtCommitReqData {
                    otid: req.otid,
                    content: next_content.into(),
                },
            }))
            .await?;
        ToentCache::refresh_chnots(&self, vec![req.otid]).await?;
        Ok(ToentTodoStateCommitRsp {
            otid: req.otid,
            todo_state: rsp
                .todo_event
                .map(|e| e.state)
                .unwrap_or(TodoStateEnum::Done),
        })
    }
}

async fn toent_todo_state_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<ToentTodoStateCommitReq>,
) -> KResponse<ToentTodoStateCommitRsp> {
    state
        .toent_todo_state_commit(kreq(headers, req))
        .await
        .into()
}

fn match_cache_filters(inst: &ToentInst, req: &ToentSearchReq) -> bool {
    if !req.include_no_time_todo && is_no_time_inst(inst) {
        return false;
    }

    let is_done = matches!(inst.todo_state, Some(TodoStateEnum::Done));
    if is_done {
        req.include_completed
    } else {
        req.include_uncompleted
    }
}

fn match_cache_range(inst: &ToentInst, req: &ToentSearchReq) -> bool {
    if is_no_time_inst(inst) {
        return req.include_no_time_todo;
    }

    let Some(start_tid) = inst.start_tid else {
        return false;
    };
    let end_tid = inst.end_tid.unwrap_or(start_tid);
    end_tid > req.start_tid && start_tid < req.end_tid
}

fn is_no_time_inst(inst: &ToentInst) -> bool {
    matches!((inst.start_tid, inst.end_tid), (None, None))
        || matches!((inst.start_tid, inst.end_tid), (Some(start), Some(end)) if start == TID::never() && end == TID::never())
}

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route(
            "/api/v1/toent-timeevent-guess",
            post(toent_time_event_guess),
        )
        .route(
            "/api/v1/toent-todoevent-guess",
            post(toent_todo_event_guess),
        )
        .route("/api/v1/toent-inst-count", post(toent_inst_count))
        .route("/api/v1/toent-search", post(toent_search))
        .route("/api/v1/toent-inst-commit", post(toent_inst_commit))
        .route(
            "/api/v1/toent-todo-state-commit",
            post(toent_todo_state_commit),
        )
}
