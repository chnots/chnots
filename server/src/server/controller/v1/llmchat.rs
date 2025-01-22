use axum::{
    extract::{Query, State},
    http::HeaderMap,
    routing::{delete, get, put},
    Json, Router,
};

use crate::{
    app::ShareAppState,
    mapper::LLMChatMapper,
    model::dto::{kreq, llmchat::*},
    server::controller::KResponse,
};

pub fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/llmchat/bot", put(bot_overwrite))
        .route("/api/v1/llmchat/bot", delete(bot_deletetion))
        .route("/api/v1/llmchat/bots", get(bot_list))
        .route("/api/v1/llmchat/template", delete(template_deletetion))
        .route("/api/v1/llmchat/template", put(template_overwrite))
        .route("/api/v1/llmchat/templates", get(template_list))
        .route("/api/v1/llmchat/session", put(session_insertion))
        .route("/api/v1/llmchat/session", delete(session_deletetion))
        .route("/api/v1/llmchat/session", get(session_detail))
        .route("/api/v1/llmchat/sessions", get(session_list))
        .route("/api/v1/llmchat/record", put(record_insertion))
}

async fn bot_overwrite(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatOverwriteBotReq>,
) -> KResponse<LLMChatOverwriteBotRsp> {
    state
        .mapper
        .llm_chat_overwrite_bot(kreq(headers, req))
        .await
        .into()
}

async fn bot_deletetion(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatDeleteBotReq>,
) -> KResponse<LLMChatDeleteBotRsp> {
    state
        .mapper
        .llm_chat_delete_bot(kreq(headers, req))
        .await
        .into()
}

async fn bot_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<LLMChatListBotReq>,
) -> KResponse<LLMChatListBotRsp> {
    state
        .mapper
        .llm_chat_list_bots(kreq(headers, req))
        .await
        .into()
}

async fn template_deletetion(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatDeleteTemplateReq>,
) -> KResponse<LLMChatDeleteTemplateRsp> {
    state
        .mapper
        .llm_chat_delete_template(kreq(headers, req))
        .await
        .into()
}

async fn template_overwrite(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatOverwriteTemplateReq>,
) -> KResponse<LLMChatOverwriteTemplateRsp> {
    state
        .mapper
        .llm_chat_overwrite_template(kreq(headers, req))
        .await
        .into()
}

async fn template_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<LLMChatListTemplateReq>,
) -> KResponse<LLMChatListTemplateRsp> {
    state
        .mapper
        .llm_chat_list_templates(kreq(headers, req))
        .await
        .into()
}

async fn session_deletetion(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatDeleteSessionReq>,
) -> KResponse<LLMChatDeleteSessionRsp> {
    state
        .mapper
        .llm_chat_delete_session(kreq(headers, req))
        .await
        .into()
}

async fn session_insertion(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatInsertSessionReq>,
) -> KResponse<LLMChatInsertSessionRsp> {
    state
        .mapper
        .llm_chat_insert_session(kreq(headers, req))
        .await
        .into()
}
async fn session_list(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<LLMChatListSessionReq>,
) -> KResponse<LLMChatListSessionRsp> {
    state
        .mapper
        .llm_chat_list_sessions(kreq(headers, req))
        .await
        .into()
}

async fn session_detail(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<LLMChatSessionDetialReq>,
) -> KResponse<LLMChatSessionDetailRsp> {
    state
        .mapper
        .llm_chat_session_detail(kreq(headers, req))
        .await
        .into()
}

async fn record_insertion(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<LLMChatInsertRecordReq>,
) -> KResponse<LLMChatInsertRecordRsp> {
    state
        .mapper
        .llm_chat_insert_record(kreq(headers, req))
        .await
        .into()
}
