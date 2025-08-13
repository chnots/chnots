use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Query, State},
    http::HeaderMap,
    routing::{get, post, put},
};

use crate::{app::ShareAppState, controller::KResponse, model::dto::kreq};

use super::{mapper::KFileMapper, *};

pub(crate) async fn query_kfile(
    state: State<ShareAppState>,
    Query(req): Query<QueryKFileReq>,
) -> KResponse<QueryKFileMetaRsp> {
    state.mapper.query_kfile_meta(req).await.into()
}

async fn query_inline_kfile(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Query(req): Query<QueryInlineKFileReq>,
) -> KResponse<QueryInlineKFileRsp> {
    state
        .mapper
        .query_inline_kfile(kreq(headers, req))
        .await
        .into()
}

async fn insert_inline_kfile(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<InsertInlineKFileReq>,
) -> KResponse<InsertInlineKFileRsp> {
    state
        .mapper
        .insert_inline_kfile(kreq(headers, req))
        .await
        .into()
}

async fn inline_kfile_insert_directly(
    state: State<ShareAppState>,
    Json(req): Json<KFileInlineInsert2Req>,
) -> KResponse<KFileInlineInsert2Rsp> {
    state
        .insert_inline_kfile2(req.file)
        .await
        .map(|_| KFileInlineInsert2Rsp {})
        .into()
}

async fn inline_kfile_get_by_sid(
    state: State<ShareAppState>,
    Query(req): Query<KFileInlineGetBySidReq>,
) -> KResponse<KFileInlineGetBySidRsp> {
    state.query_inline_kfile_by_sid(req.sid).await.into()
}

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route(
            "/api/v1/kfile/upload-by-chunks",
            post(|headers, state, mp| async {
                let rsp: KResponse<KFileUploadRsp> =
                    transfer::upload_by_chunks(headers, state, mp).await.into();
                rsp
            })
            .route_layer(DefaultBodyLimit::max(135476000)),
        )
        .route(
            "/api/v1/kfile/{meta_otid}/{filename}",
            get(transfer::download),
        )
        .route("/api/v1/kfile/info", get(query_kfile))
        .route("/api/v1/kfile/inline-upload", put(insert_inline_kfile))
        .route("/api/v1/kfile/inline-download", get(query_inline_kfile))
        .route(KFILE_INLINE_GET_BY_SID, get(inline_kfile_get_by_sid))
        .route(KFILE_INLINE_INSERT2, put(inline_kfile_insert_directly))
        .route(
            &format!("{KFILE_BIG_UPLOAD_WITH_SID}/{{sid}}"),
            post(transfer::upload_big_file_with_sid),
        )
}
