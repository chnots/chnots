use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, Query, State},
    http::HeaderMap,
    routing::{get, post, put},
};

use crate::{app::ShareAppState, controller::KResponse, model::dto::kreq};

use super::{mapper::KFileMapper, *};

pub(crate) async fn kfile_meta_fetch(
    state: State<ShareAppState>,
    Json(req): Json<KfileMetaFetchReq>,
) -> KResponse<KfileMetaFetchRsp> {
    state.mapper.query_kfile_meta(req).await.into()
}

async fn kfile_inline_download(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KfileInlineDownloadReq>,
) -> KResponse<KfileInlineDownloadRsp> {
    state
        .mapper
        .query_inline_kfile(kreq(headers, req))
        .await
        .into()
}

async fn kfile_inline_upload(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<KfileInlineUploadReq>,
) -> KResponse<KfileInlineUploadRsp> {
    state
        .mapper
        .insert_inline_kfile(kreq(headers, req))
        .await
        .into()
}

async fn kfile_inline_upload_directly(
    state: State<ShareAppState>,
    Json(req): Json<KfileInlineUploadDirectlyReq>,
) -> KResponse<KfileInlineUploadDirectlyRsp> {
    state
        .insert_inline_kfile2(req.file)
        .await
        .map(|_| KfileInlineUploadDirectlyRsp {})
        .into()
}

async fn kfile_inline_download_by_sid(
    state: State<ShareAppState>,
    Query(req): Query<KfileInlineDownloadBySidReq>,
) -> KResponse<KfileInlineDownloadBySidRsp> {
    state.query_inline_kfile_by_sid(req.sid).await.into()
}

pub(crate) fn routes() -> Router<ShareAppState> {
    Router::new()
        .route("/api/v1/kfile-meta-fetch", post(kfile_meta_fetch))
        .route(
            "/api/v1/kfile-asset-chunk-upload",
            post(|headers, state, mp| async {
                let rsp: KResponse<KFileUploadRsp> =
                    transfer::kfile_asset_chunk_upload(headers, state, mp)
                        .await
                        .into();
                rsp
            })
            .route_layer(DefaultBodyLimit::max(135476000)),
        )
        .route(
            &format!("{KFILE_ASSET_UPLOAD_BY_SID}/{{sid}}"),
            post(transfer::kfile_asset_upload_by_sid),
        )
        .route(
            "/api/v1/kfile-asset-download/{meta_otid}/{filename}",
            get(transfer::kfile_asset_download),
        )
        .route("/api/v1/kfile-inline-upload", put(kfile_inline_upload))
        .route("/api/v1/kfile-inline-download", post(kfile_inline_download))
        .route(
            KFILE_INLINE_DOWNLOAD_BY_SID,
            get(kfile_inline_download_by_sid),
        )
        .route(
            KFILE_INLINE_UPLOAD_DIRECTLY,
            put(kfile_inline_upload_directly),
        )
}
