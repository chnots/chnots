use axum::{
    Json, Router,
    extract::{Query, State},
    http::HeaderMap,
    routing::post,
};

use super::{mapper::LLMChatMapper, *};

use crate::{app::ShareAppState, controller::KResponse, model::dto::kreq};

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/llmchat-bot-commit", post(llmchat_bot_commit))
        .route("/api/v1/llmchat-bot-archive", post(llmchat_bot_archive))
        .route("/api/v1/llmchat-bot-list", post(llmchat_bot_list))
        .route(
            "/api/v1/llmchat-template-archive",
            post(llmchat_template_archive),
        )
        .route(
            "/api/v1/llmchat-template-commit",
            post(llmchat_template_commit),
        )
        .route("/api/v1/llmchat-template-list", post(llmchat_template_list))
        .route(
            "/api/v1/llmchat-session-commit",
            post(llmchat_session_commit),
        )
        .route(
            "/api/v1/llmchat-session-archive",
            post(llmchat_session_archive),
        )
        .route("/api/v1/llmchat-session-list", post(llmchat_session_list))
        .route(
            "/api/v1/llmchat-session-record-fetch",
            post(llmchat_session_record_fetch),
        )
        .route(
            "/api/v1/llmchat-session-record-truncate",
            post(llmchat_session_record_truncate),
        )
        .route("/api/v1/llmchat-record-commit", post(llmchat_record_commit))
}

async fn llmchat_bot_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatBotCommitReq>,
) -> KResponse<LLMChatBotCommitRsp> {
    state.llmchat_bot_commit(kreq(headers, req)).await.into()
}

async fn llmchat_bot_archive(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatBotArchiveReq>,
) -> KResponse<LLMChatBotArchiveRsp> {
    state.llmchat_bot_archive(kreq(headers, req)).await.into()
}

async fn llmchat_bot_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<LLMChatBotListReq>,
) -> KResponse<LLMChatBotListRsp> {
    state.llmchat_bot_list(kreq(headers, req)).await.into()
}

async fn llmchat_template_archive(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatTemplateArchiveReq>,
) -> KResponse<LLMChatTemplateArchiveRsp> {
    state
        .llmchat_template_archive(kreq(headers, req))
        .await
        .into()
}

async fn llmchat_template_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatTemplateCommitReq>,
) -> KResponse<LLMChatTemplateCommitRsp> {
    state
        .llmchat_template_commit(kreq(headers, req))
        .await
        .into()
}

async fn llmchat_template_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<LLMChatTemplateListReq>,
) -> KResponse<LLMChatTemplateListRsp> {
    state.llmchat_template_list(kreq(headers, req)).await.into()
}

async fn llmchat_session_archive(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatSessionArchiveReq>,
) -> KResponse<LLMChatSessionArchiveRsp> {
    state
        .llmchat_session_archive(kreq(headers, req))
        .await
        .into()
}

async fn llmchat_session_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatSessionCommitReq>,
) -> KResponse<LLMChatSessionCommitRsp> {
    state
        .llmchat_session_commit(kreq(headers, req))
        .await
        .into()
}
async fn llmchat_session_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<LLMChatSessionListReq>,
) -> KResponse<LLMChatSessionListRsp> {
    state.llmchat_session_list(kreq(headers, req)).await.into()
}

async fn llmchat_session_record_fetch(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<LLMChatSessionRecordFetchReq>,
) -> KResponse<LLMChatSessionRecordFetchRsp> {
    state
        .llmchat_session_record_fetch(kreq(headers, req))
        .await
        .into()
}

async fn llmchat_session_record_truncate(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatSessionRecordTruncateReq>,
) -> KResponse<LLMChatSessionRecordTruncateRsp> {
    state
        .llmchat_session_record_truncate(kreq(headers, req))
        .await
        .into()
}

async fn llmchat_record_commit(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatRecordCommitReq>,
) -> KResponse<LLMChatRecordCommitRsp> {
    state.llmchat_record_commit(kreq(headers, req)).await.into()
}
