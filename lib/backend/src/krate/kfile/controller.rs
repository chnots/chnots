use anyhow::Context;
use axum::{
    Json, Router,
    body::{self, Body},
    extract::{DefaultBodyLimit, Query, State},
    http::{HeaderMap, HeaderName},
    response::IntoResponse,
    routing::{get, post, put},
};
use chin_tools::AResult;
use reqwest::header;

use crate::{
    app::ShareAppState, controller::KResponse, model::dto::kreq, util::vec_util::RemoveNth,
};

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
    Json(req): Json<InlineKFileDownloadReq>,
) -> KResponse<InlineKFileDownloadRsp> {
    state
        .mapper
        .query_inline_kfile(kreq(headers, req))
        .await
        .into()
}

async fn kfile_inline_asset_download(
    state: State<ShareAppState>,
    axum::extract::Path((sid, filename)): axum::extract::Path<(String, String)>,
) -> impl IntoResponse {
    async fn inner(
        state: State<ShareAppState>,
        sid: String,
    ) -> AResult<([(HeaderName, String); 1], Body)> {
        let headers = [(header::CONTENT_TYPE, "plain/text".to_owned())];
        let rsp = state
            .po_inline_kfile_list(vec![sid.try_into()?])
            .await?
            .remove_n(0);
        let content = rsp.context("unable to get inline kfile")?.content;

        Ok((headers, content.to_string().into()))
    }
    match inner(state, sid).await {
        Ok(res) => Ok(res),
        Err(err) => Err((
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unable to download: {:?}, {}", &err.to_string(), err),
        )),
    }
}

async fn kfile_inline_upload(
    headers: HeaderMap,
    state: State<ShareAppState>,
    Json(req): Json<InlineKFileUploadReq>,
) -> KResponse<InlineKFileUploadRsp> {
    state
        .mapper
        .insert_inline_kfile(kreq(headers, req))
        .await
        .into()
}

async fn po_inline_kfile_commit(
    state: State<ShareAppState>,
    Json(req): Json<PoInlineKfileCommitReq>,
) -> KResponse<PoInlineKfileCommitRsp> {
    state
        .po_inline_kfile_commit(req.file)
        .await
        .map(|_| PoInlineKfileCommitRsp {})
        .into()
}

async fn po_inline_kfile_list(
    state: State<ShareAppState>,
    Query(req): Query<PoInlineKFileListReq>,
) -> KResponse<PoInlineKFileListRsp> {
    state
        .po_inline_kfile_list(req.pids)
        .await
        .map(|pos| PoInlineKFileListRsp { pos: pos })
        .into()
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
        .route(
            "/api/v1/kfile-inline-asset-download/{sid}/{filename}",
            get(kfile_inline_asset_download),
        )
        .route("/api/v1/kfile-inline-upload", put(kfile_inline_upload))
        .route("/api/v1/kfile-inline-download", post(kfile_inline_download))
        .route(PO_INLINE_KFILE_LIST, get(po_inline_kfile_list))
        .route(PO_INLINE_KFILE_COMMIT, put(po_inline_kfile_commit))
}
