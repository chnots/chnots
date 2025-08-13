use std::path::PathBuf;

use anyhow::Context;
use chin_tools::EResult;
use log::{error, info, warn};
use reqwest::{Body, multipart};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{
    app::ShareAppState,
    krate::{
        kfile::{
            KFILE_BIG_UPLOAD_WITH_SID, KFILE_INLINE_INSERT2, KFileInlineInsert2Req, KFileMeta,
            mapper::KFileMapper,
        },
        sync::{
            dto::SyncDataArg, filedumper::StartType, networksync::OtidRelatedWorker,
            po::SyncEndpoint,
        },
    },
    util::digestutil::file_blake3_sum,
};

use super::{KFileInlineGetBySidReq, KFileInlineGetBySidRsp};

struct KFileAssetWorker {
    app: ShareAppState,
}

impl KFileAssetWorker {
    async fn pull_kfile(&self, endpoint: &SyncEndpoint, list: Vec<&KFileMeta>) -> EResult {
        let client = reqwest::Client::builder().build()?;
        for kfm in list {
            if kfm.inline {
                let rsp = client
                    .get(endpoint.to_url(crate::krate::kfile::dto::KFILE_INLINE_GET_BY_SID))
                    .query(&KFileInlineGetBySidReq {
                        sid: kfm.sid.clone(),
                    })
                    .send()
                    .await?
                    .json::<KFileInlineGetBySidRsp>()
                    .await?;
                if let Some(ele) = rsp.file {
                    self.app.insert_inline_kfile2(ele).await?;
                }
            } else {
                let tmp_path = self
                    .app
                    .config
                    .attachment
                    .get_sid_path(kfm.sid.to_string() + "_downwhole");
                if tmp_path.exists() {
                    tokio::fs::remove_file(&tmp_path).await?;
                }
                super::transfer::try_mkdirp(
                    &tmp_path.parent().context("unable to get parent path")?,
                )
                .await?;
                let mut file = File::create(&tmp_path).await?;

                let rsp = client
                    .get(endpoint.to_url(format!("/api/v1/kfile/{}/{}", kfm.id, kfm.sid)))
                    .send()
                    .await?;

                if !rsp.status().is_success() {
                    warn!("rsp status is not right, {:?}", rsp.text().await);
                    return Ok(());
                }
                let mut stream = rsp.bytes_stream();

                while let Some(chunk_result) = futures::StreamExt::next(&mut stream).await {
                    let chunk = chunk_result?;
                    file.write_all(&chunk).await?;
                }

                file.flush().await?;

                let sum = file_blake3_sum(&tmp_path)?;
                let final_path = self.app.config.attachment.get_sid_path(kfm.sid.as_str());

                if sum != kfm.sid.as_str() {
                    warn!(
                        "the pulled file is not correct. filename {}, blake3sum db: {} -- file: {}",
                        kfm.filename, kfm.sid, sum
                    );
                    if file.metadata().await?.len() != (kfm.filesize as u64) {
                        anyhow::bail!(
                            "event the file size is not same: (file){} : (db){}, filepath {:?}",
                            file.metadata().await?.len(),
                            kfm.filesize,
                            tmp_path
                        );
                    }
                }
                tokio::fs::rename(tmp_path, final_path).await?;
            }
        }

        Ok(())
    }

    async fn push_kfile(&self, endpoint: &SyncEndpoint, list: Vec<&KFileMeta>) -> EResult {
        let client = reqwest::Client::builder().build()?;

        for kfm in list {
            if kfm.inline {
                let rsp = self.app.query_inline_kfile_by_sid(kfm.sid.clone()).await?;
                if let Some(c) = rsp.file {
                    client
                        .put(endpoint.to_url(KFILE_INLINE_INSERT2))
                        .json(&KFileInlineInsert2Req { file: c })
                        .send()
                        .await?;
                }
            } else {
                let path = self.app.config.attachment.get_sid_path(kfm.sid.as_str());
                let file_size = path.metadata()?.len();
                // https://stackoverflow.com/questions/65814450/how-to-post-a-file-using-reqwest
                let file = match File::open(&path).await {
                    Ok(file) => file,
                    Err(te) => {
                        error!("open file error: {path:?} {te}");
                        continue;
                    }
                };

                // read file body stream
                let stream = FramedRead::new(file, BytesCodec::new());
                let file_body = Body::wrap_stream(stream);

                //make form part of file
                let some_file = multipart::Part::stream(file_body)
                    .file_name(kfm.filename.to_string())
                    .mime_str("text/plain")?;

                //create the multipart form
                let form = multipart::Form::new().part("file", some_file);

                //send request
                let response = client
                    .post(
                        endpoint
                            .to_url(format!("{KFILE_BIG_UPLOAD_WITH_SID}/{}", kfm.sid.as_str())),
                    )
                    .multipart(form)
                    .header("K-filesize", file_size)
                    .send()
                    .await?;
                let result = response.text().await?;
                info!("upload file result {result}");
            }
        }

        Ok(())
    }
}

impl OtidRelatedWorker<KFileMeta> for KFileAssetWorker {
    async fn before_send(&self, endpoint: &SyncEndpoint, arg: &SyncDataArg<KFileMeta>) -> EResult {
        for ele in &arg.cmds {
            if let crate::krate::sync::dto::SyncDataOperation::Push { data, hist: _ } = ele {
                self.push_kfile(endpoint, vec![data]).await?;
            }
        }

        Ok(())
    }

    async fn before_merge(&self, endpoint: &SyncEndpoint, arg: &SyncDataArg<KFileMeta>) -> EResult {
        for ele in &arg.cmds {
            if let crate::krate::sync::dto::SyncDataOperation::Push { data, hist: _ } = ele {
                self.pull_kfile(endpoint, vec![data]).await?;
            }
        }

        Ok(())
    }
}

impl ShareAppState {
    pub async fn dump_kfile_to_file(&self, start_type: StartType, backup_dir: &PathBuf) -> EResult {
        self.mapper
            .dump_to_file::<&PathBuf, KFileMeta>(backup_dir, start_type)
            .await?;
        // TODO
        Ok(())
    }

    pub async fn sync_kfile(&self, endpoint: &SyncEndpoint) -> EResult {
        let worker = KFileAssetWorker { app: self.clone() };
        self.sync_one_otid_table(std::marker::PhantomData::<KFileMeta>, endpoint, &worker)
            .await?;

        Ok(())
    }
}
