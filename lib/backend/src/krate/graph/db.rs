use std::collections::BTreeMap;

use crate::{
    krate::graph::{
        ExcalidrawDataV2, ExcalidrawDataV2Dto, ExcalidrawDataV2Po, ExcalidrawFetchRsp,
        ExcalidrawHistoryApplyReq, ExcalidrawHistoryApplyRsp, ExcalidrawHistoryFetchReq,
        ExcalidrawHistoryFetchRsp, ExcalidrawLibraryFetchRsp, ExcalidrawLibraryMetaV1, GetKeys,
        GraphData, GraphHistoryListReq, GraphHistoryListRsp, GraphHistoryVersion, GraphKind,
        GraphMeta, MindElixirDataV1Po, MindElixirDataV1PoMeta, MindElixirHistoryApplyReq,
        MindElixirHistoryApplyRsp, MindElixirHistoryFetchReq, MindElixirHistoryFetchRsp,
        MindElixirLoadRsp, mapper::GraphMapper,
    },
    mapper::{
        Curd,
        db::{
            HistCreateSql, KDbConnBehaiver, KDbExecutor, KDbRow, KDbTransactionBehaiver, KDbTx,
            helper::{Ddls, print_ddls},
        },
    },
    model::{dto::KReq, otid_table::OtidSearchType},
};
use chin_tools::{AResult, EResult};
use serde_json::Value;

use crate::util::digestutil::blake3_sum16;

use crate::mapper::db::{KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRowBehavier};

use chin_sql::{SqlBuilder, Wheres, time_type::TID};

impl KDbExecutor<'_> {
    async fn po_query_graph_meta(&self, otid: TID) -> anyhow::Result<Option<GraphMeta>> {
        let c = self
            .qry_opt(GraphMeta::pkey_reader(otid), |e| (&e).try_into())
            .await?;
        Ok(c)
    }
    async fn query_graph_data(
        &self,
        sids: Vec<&str>,
    ) -> anyhow::Result<BTreeMap<String, GraphData>> {
        let c: Vec<GraphData> = self
            .qry_list(
                SqlBuilder::read_all(GraphData::TABLE).r#where(Wheres::r#in(GraphData::SID, sids)),
                |e| (&e).try_into(),
            )
            .await?;
        let c = c.into_iter().map(|e| (e.sid.to_string(), e)).collect();
        Ok(c)
    }

    async fn load_excalidraw_data_by_meta_content(
        &self,
        content: &str,
    ) -> anyhow::Result<ExcalidrawDataV2Dto> {
        let content: ExcalidrawDataV2<String> = serde_json::from_str(content)?;
        let keys = content.get_keys();
        let data = self
            .query_graph_data(keys.iter().map(|e| e.as_str()).collect())
            .await?;

        let mut others: BTreeMap<String, Value> = BTreeMap::new();
        for (k, v) in content.others {
            others.insert(
                k,
                serde_json::from_str(
                    data.get(&v)
                        .ok_or(anyhow::anyhow!("unable to find {}", v))?
                        .content
                        .as_str(),
                )?,
            );
        }

        let mut elements = vec![];
        for ele in content.elements {
            elements.push(serde_json::from_str(
                data.get(&ele)
                    .ok_or(anyhow::anyhow!("unable to find {}", ele))?
                    .content
                    .as_str(),
            )?);
        }

        Ok(ExcalidrawDataV2Dto(ExcalidrawDataV2 { others, elements }))
    }

    async fn load_mind_elixir_po_by_meta_content(
        &self,
        content: &str,
    ) -> anyhow::Result<MindElixirDataV1Po> {
        let meta: MindElixirDataV1PoMeta = serde_json::from_str(content)?;
        let data = self
            .query_graph_data(meta.keys.iter().map(|e| e.as_str()).collect())
            .await?
            .into_iter()
            .map(|(k, v)| (k, v.content.into()))
            .collect();

        Ok(MindElixirDataV1Po { meta, data })
    }

    async fn query_graph_history_versions(&self, otid: TID) -> AResult<Vec<GraphHistoryVersion>> {
        let versions = self
            .po_otid_tid_list::<GraphMeta, _>([otid], OtidSearchType::Hist)
            .await?
            .into_iter()
            .map(|e| GraphHistoryVersion { tid: e })
            .collect();

        Ok(versions)
    }

    async fn query_graph_history_meta_by_tid<F>(
        &self,
        otid: TID,
        tid: TID,
        kind_match: F,
    ) -> AResult<Option<GraphMeta>>
    where
        F: Fn(&GraphKind) -> bool,
    {
        let meta = self
            .po_otid_fetch_by_tid::<GraphMeta>(tid, OtidSearchType::Hist)
            .await?;

        Ok(meta.filter(|m| m.otid == otid && kind_match(&m.kind)))
    }
}

impl KDbTx<'_> {
    async fn po_insert_graph_meta(&self, graph: GraphMeta) -> anyhow::Result<()> {
        self.po_otid_commit([graph]).await?;
        Ok(())
    }

    async fn po_insert_graph_data(&self, graph: GraphData) -> anyhow::Result<()> {
        self.exec(
            graph
                .to_sql_inserter()
                .on_conflict(chin_sql::OnConflict::Ignore),
        )
        .await?;
        Ok(())
    }

    async fn po_insert_graph_snapshot(
        &self,
        otid: TID,
        kind: GraphKind,
        meta_content: String,
        data: BTreeMap<String, String>,
    ) -> EResult {
        self.po_insert_graph_meta(GraphMeta {
            otid,
            archor: false,
            kind,
            content: meta_content.into(),
            tid: TID::now(),
        })
        .await?;

        for (k, v) in data {
            self.po_insert_graph_data(GraphData {
                sid: k.try_into()?,
                tid: TID::now(),
                content: v.into(),
            })
            .await?;
        }

        Ok(())
    }

    pub async fn excalidraw_commit(&self, po: ExcalidrawDataV2Po, otid: TID) -> EResult {
        self.po_insert_graph_snapshot(
            otid,
            super::GraphKind::ExcalidrawV2,
            serde_json::to_string(&po.meta)?,
            po.data,
        )
        .await?;

        Ok(())
    }

    async fn excalidraw_library_commit(&self, otid: TID, data: Value) -> EResult {
        let content = serde_json::to_string(&data)?;
        let sid = blake3_sum16(content.as_bytes())?;
        let meta = ExcalidrawLibraryMetaV1 { sid };

        self.po_insert_graph_meta(GraphMeta {
            otid,
            archor: false,
            kind: super::GraphKind::ExcalidrawLibraryV1,
            content: serde_json::to_string(&meta)?.into(),
            tid: TID::now(),
        })
        .await?;

        self.po_insert_graph_data(GraphData {
            sid: meta.sid.try_into()?,
            tid: TID::now(),
            content: content.into(),
        })
        .await?;

        Ok(())
    }
}

impl Curd for GraphMeta {
    fn pkey(&self) -> Wheres<'_> {
        Self::pkey_cond(self.otid)
    }
    fn tid(&self) -> TID {
        self.tid
    }
}

impl TryFrom<&KDbRow> for GraphMeta {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let chnot = GraphMeta {
            otid: value.try_get(GraphMeta::OTID)?,
            tid: value.try_get(GraphMeta::TID)?,
            archor: value.try_get(GraphMeta::ARCHOR)?,
            kind: value.try_get(GraphMeta::KIND)?,
            content: value.try_get(GraphMeta::CONTENT)?,
        };
        Ok(chnot)
    }
}

impl TryFrom<&KDbRow> for GraphData {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let chnot = GraphData {
            tid: value.try_get(GraphData::TID)?,
            sid: value.try_get(GraphData::SID)?,
            content: value.try_get(GraphData::CONTENT)?,
        };
        Ok(chnot)
    }
}

impl GraphMapper for KDb {
    async fn excalidraw_fetch(
        &self,
        req: KReq<super::ExcalidrawFetchReq>,
    ) -> chin_tools::AResult<super::ExcalidrawFetchRsp> {
        let conn = self.conn().await?;
        let meta = conn.as_executor().po_query_graph_meta(req.otid).await?;
        let Some(meta) = meta else {
            return Ok(ExcalidrawFetchRsp { data: None });
        };
        let data = conn
            .as_executor()
            .load_excalidraw_data_by_meta_content(meta.content.as_str())
            .await?;

        Ok(ExcalidrawFetchRsp { data: Some(data) })
    }

    async fn excalidraw_history_list(
        &self,
        req: KReq<GraphHistoryListReq>,
    ) -> AResult<GraphHistoryListRsp> {
        let conn = self.conn().await?;
        let versions = conn
            .as_executor()
            .query_graph_history_versions(req.body.otid)
            .await?;

        Ok(GraphHistoryListRsp { versions })
    }

    async fn excalidraw_history_fetch(
        &self,
        req: KReq<ExcalidrawHistoryFetchReq>,
    ) -> AResult<ExcalidrawHistoryFetchRsp> {
        let conn = self.conn().await?;
        let meta = conn
            .as_executor()
            .query_graph_history_meta_by_tid(req.body.otid, req.body.tid, |k| {
                matches!(k, GraphKind::ExcalidrawV2)
            })
            .await?;
        let Some(meta) = meta else {
            return Ok(ExcalidrawHistoryFetchRsp { data: None });
        };
        let data = conn
            .as_executor()
            .load_excalidraw_data_by_meta_content(meta.content.as_str())
            .await?;

        Ok(ExcalidrawHistoryFetchRsp { data: Some(data) })
    }

    async fn excalidraw_history_apply(
        &self,
        req: KReq<ExcalidrawHistoryApplyReq>,
    ) -> AResult<ExcalidrawHistoryApplyRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;

        let history = tx
            .as_executor()
            .query_graph_history_meta_by_tid(req.body.otid, req.body.tid, |k| {
                matches!(k, GraphKind::ExcalidrawV2)
            })
            .await?;
        let Some(history) = history else {
            return Ok(ExcalidrawHistoryApplyRsp { data: None });
        };

        let data = tx
            .as_executor()
            .load_excalidraw_data_by_meta_content(history.content.as_str())
            .await?;
        tx.po_otid_commit_by_tid::<GraphMeta>(history.tid).await?;
        tx.cmt().await?;

        Ok(ExcalidrawHistoryApplyRsp { data: Some(data) })
    }

    async fn excalidraw_commit(
        &self,
        req: KReq<super::ExcalidrawCommitReq>,
    ) -> chin_tools::AResult<super::ExcalidrawCommitRsp> {
        let otid = req.otid;
        let po: ExcalidrawDataV2Po = req.body.data.try_into()?;
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.excalidraw_commit(po, otid).await?;
        tx.cmt().await?;

        Ok(super::ExcalidrawCommitRsp {})
    }

    async fn excalidraw_library_fetch(
        &self,
        req: KReq<super::ExcalidrawLibraryFetchReq>,
    ) -> chin_tools::AResult<super::ExcalidrawLibraryFetchRsp> {
        let conn = self.conn().await?;
        let meta = conn.as_executor().po_query_graph_meta(req.otid).await?;
        let Some(meta) = meta else {
            return Ok(ExcalidrawLibraryFetchRsp { data: None });
        };
        if !matches!(meta.kind, super::GraphKind::ExcalidrawLibraryV1) {
            return Ok(ExcalidrawLibraryFetchRsp { data: None });
        }

        let meta: ExcalidrawLibraryMetaV1 = serde_json::from_str(meta.content.as_str())?;
        let data = conn
            .as_executor()
            .query_graph_data(vec![meta.sid.as_str()])
            .await?;

        let Some(data) = data.get(meta.sid.as_str()) else {
            return Ok(ExcalidrawLibraryFetchRsp { data: None });
        };

        Ok(ExcalidrawLibraryFetchRsp {
            data: Some(serde_json::from_str(data.content.as_str())?),
        })
    }

    async fn excalidraw_library_commit(
        &self,
        req: KReq<super::ExcalidrawLibraryCommitReq>,
    ) -> chin_tools::AResult<super::ExcalidrawLibraryCommitRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.excalidraw_library_commit(req.otid, req.body.data)
            .await?;
        tx.cmt().await?;

        Ok(super::ExcalidrawLibraryCommitRsp {})
    }

    async fn mind_elixir_fetch(
        &self,
        req: KReq<super::MindElixirLoadReq>,
    ) -> chin_tools::AResult<super::MindElixirLoadRsp> {
        let conn = self.conn().await?;
        let meta = conn.as_executor().po_query_graph_meta(req.otid).await?;
        let Some(meta) = meta else {
            return Ok(MindElixirLoadRsp { data: None });
        };
        let po = conn
            .as_executor()
            .load_mind_elixir_po_by_meta_content(meta.content.as_str())
            .await?;

        Ok(MindElixirLoadRsp {
            data: Some(po.try_into()?),
        })
    }

    async fn mind_elixir_history_list(
        &self,
        req: KReq<GraphHistoryListReq>,
    ) -> AResult<GraphHistoryListRsp> {
        let conn = self.conn().await?;
        let versions = conn
            .as_executor()
            .query_graph_history_versions(req.body.otid)
            .await?;

        Ok(GraphHistoryListRsp { versions })
    }

    async fn mind_elixir_history_fetch(
        &self,
        req: KReq<MindElixirHistoryFetchReq>,
    ) -> AResult<MindElixirHistoryFetchRsp> {
        let conn = self.conn().await?;
        let meta = conn
            .as_executor()
            .query_graph_history_meta_by_tid(req.body.otid, req.body.tid, |k| {
                matches!(k, GraphKind::MindElixirV1)
            })
            .await?;
        let Some(meta) = meta else {
            return Ok(MindElixirHistoryFetchRsp { data: None });
        };

        let po = conn
            .as_executor()
            .load_mind_elixir_po_by_meta_content(meta.content.as_str())
            .await?;

        Ok(MindElixirHistoryFetchRsp {
            data: Some(po.try_into()?),
        })
    }

    async fn mind_elixir_history_apply(
        &self,
        req: KReq<MindElixirHistoryApplyReq>,
    ) -> AResult<MindElixirHistoryApplyRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;

        let history = tx
            .as_executor()
            .query_graph_history_meta_by_tid(req.body.otid, req.body.tid, |k| {
                matches!(k, GraphKind::MindElixirV1)
            })
            .await?;
        let Some(history) = history else {
            return Ok(MindElixirHistoryApplyRsp { data: None });
        };

        let po = tx
            .as_executor()
            .load_mind_elixir_po_by_meta_content(history.content.as_str())
            .await?;

        tx.po_otid_commit_by_tid::<GraphMeta>(history.tid).await?;
        tx.cmt().await?;

        Ok(MindElixirHistoryApplyRsp {
            data: Some(po.try_into()?),
        })
    }

    async fn mind_elixir_commit(
        &self,
        req: KReq<super::MindElixirCommitReq>,
    ) -> chin_tools::AResult<super::MindElixirCommitRsp> {
        let otid = req.otid;
        let po: MindElixirDataV1Po = req.body.data.try_into()?;

        let mut conn = self.conn().await?;
        let tx = conn.tx().await?;
        tx.po_insert_graph_snapshot(
            otid,
            super::GraphKind::MindElixirV1,
            serde_json::to_string(&po.meta)?,
            po.data,
        )
        .await?;
        tx.cmt().await?;

        Ok(super::MindElixirCommitRsp {})
    }

    async fn ensure_table_graph(&self) -> EResult {
        print_ddls(
            Ddls::new()
                .with_ddl(GraphData::create_sql().to_owned_sql())
                .with_ddls(GraphMeta::ddls()),
            self,
        )
        .await?;

        Ok(())
    }
}
