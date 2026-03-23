use std::collections::BTreeMap;

use crate::{
    krate::graph::{
        ExcalidrawDataV2, ExcalidrawDataV2Dto, ExcalidrawDataV2Po, ExcalidrawFetchRsp,
        ExcalidrawLibraryFetchRsp, ExcalidrawLibraryMetaV1, GetKeys, GraphData, GraphMeta,
        MindElixirDataV1Po, MindElixirDataV1PoMeta, MindElixirLoadRsp, mapper::GraphMapper,
    },
    mapper::{
        Curd,
        db::{
            HistCreateSql, KDbConnBehaiver, KDbExecutor, KDbRow, KDbTransactionBehaiver, KDbTx,
            helper::{Ddls, print_ddls},
        },
    },
    model::dto::KReq,
};
use chin_tools::EResult;
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

    pub async fn excalidraw_commit(&self, po: ExcalidrawDataV2Po, otid: TID) -> EResult {
        self.po_insert_graph_meta(GraphMeta {
            otid,
            // TODO
            archor: false,
            kind: super::GraphKind::ExcalidrawV2,
            content: serde_json::to_string(&po.meta)?.into(),
            tid: TID::now(),
        })
        .await?;
        for (k, v) in po.data {
            self.po_insert_graph_data(GraphData {
                sid: k.try_into()?,
                tid: TID::now(),
                content: v.into(),
            })
            .await?;
        }
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

        let content: ExcalidrawDataV2<String> = serde_json::from_str(meta.content.as_str())?;
        let keys = content.get_keys();
        let data = conn
            .as_executor()
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

        Ok(ExcalidrawFetchRsp {
            data: Some(ExcalidrawDataV2Dto(ExcalidrawDataV2 { others, elements })),
        })
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

        let meta: MindElixirDataV1PoMeta = serde_json::from_str(meta.content.as_str())?;
        let data = conn
            .as_executor()
            .query_graph_data(meta.keys.iter().map(|e| e.as_str()).collect())
            .await?
            .into_iter()
            .map(|(k, v)| (k, v.content.into()))
            .collect();

        let po = MindElixirDataV1Po { meta, data };

        Ok(MindElixirLoadRsp {
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
        tx.po_insert_graph_meta(GraphMeta {
            otid,
            // TODO
            archor: false,
            kind: super::GraphKind::MindElixirV1,
            content: serde_json::to_string(&po.meta)?.into(),
            tid: TID::now(),
        })
        .await?;
        for (k, v) in po.data {
            tx.po_insert_graph_data(GraphData {
                sid: k.try_into()?,
                tid: TID::now(),
                content: v.into(),
            })
            .await?;
        }
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
