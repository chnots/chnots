use std::collections::HashMap;

use super::mapper::ChnotMapper;
use super::*;
use crate::krate::llmchat::LLMChatRecord;
use crate::krate::mdwt::db::MdwtOtidInTags;
use crate::krate::mdwt::{MdwtRecord, MdwtRecordTable};
use crate::mapper::Curd;
use crate::mapper::db::helper::{Ddls, create_tables};
use crate::mapper::db::{
    HistCreateSql, KDb, KDbBehaiver, KDbConnBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
    KDbTransactionBehaiver, PageReader,
};
use crate::model::KSerde;
use crate::model::dto::{KReq, PageRsp};
use crate::util::string_util::StringUtils;
use chin_sql::str_type::{Text, Varchar};
use chin_sql::time_type::TID;
use chin_sql::{Froms, GenerateTableSchema, LimitOffset, SqlField, SubQueryTable, Wheres};
use chin_sql::{ILikeType, JoinTable, JoinType, Joins, OrderBy, SqlBuilder, SqlReader};
use chin_tools::{AResult, EResult};
use chrono::Local;

#[allow(dead_code)]
#[derive(Debug, Clone, GenerateTableSchema)]
struct QueryContent {
    #[gts_type = "i64"]
    chnot_otid: TID,
    content: Text,
}

impl<'a> QueryContentTable<'a> {
    fn sub_query_table(query: Option<&'a str>) -> Option<SubQueryTable<'a>> {
        if let Some(query) = query.as_ref()
            && !query.is_empty()
        {
            let mdwt = SqlReader::builder(
                [
                    SqlField {
                        alias: QueryContent::CHNOT_OTID.into(),
                        table_alias: "mr",
                        field_name: MdwtRecord::OTID,
                    },
                    SqlField {
                        alias: QueryContent::CONTENT.into(),
                        table_alias: "mr",
                        field_name: MdwtRecord::CONTENT,
                    },
                ],
                chin_sql::Froms::Table {
                    table_name: MdwtRecord::TABLE,
                    alias: "mr",
                },
            )
            .wheres(Wheres::ilike(MdwtRecord::CONTENT, query, ILikeType::Fuzzy))
            .build();

            let llmchat = SqlReader::builder(
                [
                    SqlField {
                        alias: QueryContent::CHNOT_OTID.into(),
                        table_alias: "llm",
                        field_name: LLMChatRecord::OTID,
                    },
                    SqlField {
                        alias: QueryContent::CONTENT.into(),
                        table_alias: "llm",
                        field_name: LLMChatRecord::CONTENT,
                    },
                ],
                chin_sql::Froms::Table {
                    table_name: LLMChatRecord::TABLE,
                    alias: "llm",
                },
            )
            .wheres(Wheres::ilike(
                LLMChatRecord::CONTENT,
                query,
                ILikeType::Fuzzy,
            ))
            .build();

            Some(SubQueryTable {
                reader: SqlReader::builder(
                    [
                        SqlField {
                            alias: None,
                            table_alias: "cont",
                            field_name: QueryContent::CHNOT_OTID,
                        },
                        SqlField {
                            alias: None,
                            table_alias: "cont",
                            field_name: QueryContent::CONTENT,
                        },
                    ],
                    chin_sql::Froms::Union {
                        table: [mdwt, llmchat].into(),
                        alias: "cont",
                    },
                )
                .build(),
            })
        } else {
            None
        }
    }
}

/// ```emacs-lisp
/// (let ((ctm ChnotThreadMeta)
///       (cto ChnotThreadOrder)
///       (cm  ChnotMeta)
///       (mr MdwtRecord)
///       (lcr LLMChatRecord)
///       (cotid (if with-thread cto.otid cm.otid)) ; chnot otid
///       (kspace (if with-thread ctm.kspace cm.otid))
///       )
///   (select (if with-thread
///               ())
///           (from (if with-thread
///                     (join ctm
///                           (left-join cto (ctm.otid cto.thread_otid))
///                           (when with-tag (inner-join mt (mt.otid cto.otid))))
///                   (join cm
///                         (when with-tag (inner-join mt (mt.otid cm.otid)))))
///
///
///                 ;; query part
///                 (when query
///                   (inner-join
///                    (union (select (content, chnot_otid)
///                                   mr
///                                   (where (ilike content query)))
///                           (select (content, chnot_otid)
///                                   lcr
///                                   (where (ilike content query))))))
///                 ;; used for the title
///                 (left-join mr (cm.otid mr.otid)))
///           (where
///            (when with-thread
///              (= cto.korder 0))
///            (in kspace kspaces))
///           (order
///            (if with-thread
///                ((desc ctm.pin_tid)
///                 (desc ctm.otid)
///                 (asc cto.korder))
///              ((desc cm.pin_tid)
///               (desc cm.otid))))))
/// ``
impl ChnotMapper for KDb {
    async fn ensure_table_chnot(&self) -> EResult {
        create_tables(
            Ddls::new()
                .with_ddls(ChnotThreadOrder::ddls())
                .with_ddls(ChnotThreadMeta::ddls())
                .with_ddls(ChnotMeta::ddls()),
            self,
        )
        .await
    }

    async fn chnot_thread_meta_commit(
        &self,
        req: KReq<ChnotThreadMetaFetchCommitReq>,
    ) -> AResult<ChnotThreadMetaFetchCommitRsp> {
        let mut conn = self.conn().await?;

        let tx = conn.tx().await?;

        let reader = ChnotThreadMeta::pkey_reader(req.meta_otid);
        let mut meta: ChnotThreadMeta =
            tx.qry_opt(reader, |e| (&e).try_into())
                .await?
                .unwrap_or(ChnotThreadMeta {
                    otid: req.meta_otid,
                    kspace: req.kspace.clone(),
                    pin_tid: None,
                    archive_tid: None,
                    tid: TID::default(),
                });

        tx.as_executor()
            .omit_rows::<ChnotThreadMeta>(ChnotThreadMeta::pkey_cond(req.meta_otid))
            .await?;

        meta.tid = TID::default();
        if let Some(pin_it) = req.pinned {
            if pin_it {
                meta.pin_tid = Some(TID::default());
            } else {
                meta.pin_tid = None;
            }
        }

        if let Some(archive_it) = req.archive {
            if archive_it {
                meta.archive_tid = Some(TID::default());
            } else {
                meta.archive_tid = None;
            }
        }

        if let Some(ksapce) = req.body.kspace {
            meta.kspace = ksapce;
        }

        tx.exec(meta.clone().to_sql_inserter()).await?;

        tx.cmt().await?;

        Ok(ChnotThreadMetaFetchCommitRsp { meta })
    }

    async fn chnot_overwrite_thread_orders(
        &self,
        req: KReq<ChnotThreadOrderCommitReq>,
    ) -> AResult<ChnotThreadOrderCommitRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.transaction().await?;
        let ChnotThreadOrderCommitReq {
            thread_otid,
            orders,
        } = req.body;

        let mut metas = vec![];
        for (c, b) in orders.iter().enumerate() {
            let rec = ChnotThreadOrder {
                otid: b.otid,
                tid: TID::default(),
                thread_otid,
                korder: c.try_into()?,
            };

            metas.push(rec.clone());

            tx.as_executor()
                .omit_rows::<ChnotThreadOrder>(rec.pkey())
                .await?;
            tx.exec(rec.to_sql_inserter()).await?;
        }

        tx.cmt().await?;
        Ok(ChnotThreadOrderCommitRsp {})
    }

    async fn chnot_thread_meta_fetch(
        &self,
        req: KReq<ChnotThreadMetaFetchReq>,
    ) -> AResult<ChnotThreadMetaFetchRsp> {
        let conn = self.conn().await?;
        let thread_meta = conn
            .qry_opt(ChnotThreadMeta::pkey_reader(req.thread_otid), |e| {
                ChnotThreadMeta::try_from(&e)
            })
            .await?;

        let metas = conn
            .qry_list(
                SqlBuilder::read(
                    ChnotMeta::TABLE,
                    &[format!("{}.*", ChnotMeta::TABLE).as_str()],
                )
                .seg(format!(
                    " left join {} on {}.{} = {}.{} ",
                    ChnotThreadOrder::TABLE,
                    ChnotMeta::TABLE,
                    ChnotMeta::OTID,
                    ChnotThreadOrder::TABLE,
                    ChnotThreadOrder::OTID
                ))
                .r#where(Wheres::and([Wheres::equal(
                    ChnotThreadOrder::THREAD_OTID.prefix_with_sep(ChnotThreadOrder::TABLE, "."),
                    req.thread_otid,
                )])),
                |row| ChnotMeta::try_from(&row),
            )
            .await?;
        /*         let toents = conn
        .qry_list(
            SqlBuilder::read_all(MdwtToent::TABLE).r#where(Wheres::equal(
                MdwtToent::CHNOT_OTID,
                req.chnot_meta_otid,
            )),
            |row| MdwtToent::try_from(&row),
        )
        .await?; */

        Ok(ChnotThreadMetaFetchRsp {
            thread_meta,
            chnot_meta_sorted: metas,
        })
    }

    async fn chnot_meta_commit(
        &self,
        req: KReq<ChnotMetaCommitReq>,
    ) -> AResult<ChnotMetaCommitRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.transaction().await?;
        let ChnotMetaCommitReq { metas } = req.body;

        let mut result_metas = vec![];
        for b in metas {
            let rec = ChnotMeta {
                otid: b.otid,
                tid: TID::default(),
                kind: b.kind,
                kspace: b.kspace,
                archive_tid: if b.archive.is_some_and(|v| v) {
                    Some(TID::default())
                } else {
                    None
                },
                pin_tid: if b.pin_it.is_some_and(|v| v) {
                    Some(TID::default())
                } else {
                    None
                },
            };

            result_metas.push(rec.clone());

            tx.as_executor().omit_rows::<ChnotMeta>(rec.pkey()).await?;
            tx.exec(rec.to_sql_inserter()).await?;
        }

        tx.cmt().await?;
        Ok(ChnotMetaCommitRsp {
            metas: result_metas,
        })
    }

    async fn chnot_meta_list(&self, req: KReq<ChnotMetaListReq>) -> AResult<ChnotMetaListRsp> {
        if req.body.otids.is_empty() {
            return Ok(ChnotMetaListRsp { metas: vec![] });
        }
        let cm = ChnotMetaTable::new("cm");
        let metas = SqlBuilder::read_all(&cm.nwa()).r#where(cm.otid().v_in(req.body.otids));
        let cms = self
            .conn()
            .await?
            .qry_list(metas, |row| ChnotMeta::try_from_kdb_row(&row))
            .await?;
        Ok(ChnotMetaListRsp { metas: cms })
    }

    async fn chnot_thread_search(
        &self,
        req: KReq<ChnotSearchReq>,
    ) -> AResult<PageRsp<ChnotSearchRspThread>> {
        let ctm = ChnotThreadMetaTable::new("ctm");
        let cto = ChnotThreadOrderTable::new("cto");
        let with_cto = req.query.as_ref().is_some_and(|s| !s.is_empty()) || req.tags.is_some();

        let tag_constraint = MdwtOtidInTags::new("tag_constraint");
        let query_constraint = QueryContentTable::new("query");

        let fetch_metas = SqlReader::builder(
            // select
            [SqlField {
                alias: None,
                table_alias: ctm.alias,
                field_name: "*",
            }],
            // from
            // Chnot Thread Meta
            Joins::new(chin_sql::Froms::Table {
                table_name: ctm.table(),
                alias: ctm.alias,
            })
            // Chnot Thread Order
            .join_if(
                with_cto,
                JoinTable {
                    join_type: JoinType::LeftJoin,
                    table: chin_sql::Froms::Table {
                        table_name: cto.table(),
                        alias: cto.alias,
                    },
                    conds: [(ctm.otid(), cto.thread_otid()).into()].into(),
                },
            )
            .join_some(
                tag_constraint.sub_query_table(req.tags.clone(), req.get_spaces()),
                |v| JoinTable {
                    join_type: JoinType::InnerJoin,
                    table: Froms::SubQuery {
                        table: v.reader.into(),
                        alias: &tag_constraint.alias,
                    },
                    conds: [(tag_constraint.mdwt_otid(), cto.otid()).into()].into(),
                },
            )
            .join_some(
                QueryContentTable::sub_query_table(req.query.as_deref()),
                |v| JoinTable {
                    join_type: JoinType::InnerJoin,
                    table: Froms::SubQuery {
                        table: v.reader.into(),
                        alias: query_constraint.alias,
                    },
                    conds: [(query_constraint.chnot_otid(), cto.otid()).into()].into(),
                },
            )
            .into(),
        )
        .wheres(Wheres::and([ctm.kspace().v_in(req.get_spaces())]))
        .order_by([
            OrderBy::Desc(ctm.pin_tid().twn()),
            OrderBy::Desc(ctm.otid().twn()),
            if with_cto {
                OrderBy::Asc(cto.korder().twn())
            } else {
                OrderBy::None
            },
        ])
        .build();

        let mut metas = self
            .conn()
            .await?
            .page_read(
                fetch_metas,
                LimitOffset {
                    limit: req.page_size,
                    offset: req.start_index.into(),
                },
                |row| {
                    Ok(ChnotSearchRspThread {
                        meta: ChnotThreadMeta {
                            otid: row.try_get(ChnotThreadMeta::OTID)?,
                            kspace: row.try_get(ChnotThreadMeta::KSPACE)?,
                            pin_tid: row.try_get(ChnotThreadMeta::PIN_TID)?,
                            archive_tid: row.try_get(ChnotThreadMeta::ARCHIVE_TID)?,
                            tid: row.try_get(ChnotThreadMeta::TID)?,
                        },
                        title: None,
                    })
                },
            )
            .await?;

        let mr = MdwtRecordTable::new("mr");
        let fetch_titles = SqlReader::builder(
            [cto.thread_otid().erased(), mr.content().erased()],
            Joins::new(Froms::Table {
                table_name: cto.table(),
                alias: cto.alias,
            })
            .join(JoinTable {
                join_type: JoinType::LeftJoin,
                table: Froms::Table {
                    table_name: mr.table(),
                    alias: mr.alias,
                },
                conds: [(mr.otid(), cto.otid()).into()].into(),
            })
            .into(),
        )
        .wheres(Wheres::and([
            // we only focus on the first chnot
            cto.korder().v_eq(0),
            // limit thread otids
            cto.thread_otid()
                .v_in(metas.data.iter().map(|m| m.meta.otid).collect()),
        ]))
        .build();
        let content = mr.content().field_name;
        let otid = cto.thread_otid().field_name;

        let titles: Vec<(TID, String)> = self
            .conn()
            .await?
            .qry_list(fetch_titles, |row| {
                let otid: TID = row.try_get(otid)?;
                let content: String = row.try_get(content)?;
                Ok((otid, content))
            })
            .await?;
        let mut titles: HashMap<TID, String> = titles.into_iter().collect();
        metas.data.iter_mut().for_each(|m| {
            if let Some(title) = titles.remove(&m.meta.otid) {
                m.title.replace(title);
            }
        });

        Ok(metas)
    }

    async fn chnot_single_search(
        &self,
        req: KReq<ChnotSearchReq>,
    ) -> AResult<PageRsp<ChnotSearchRspSingle>> {
        let cm = ChnotMetaTable::new("cm");
        let tag_constraint = MdwtOtidInTags::new("tag_constraint");
        let query_constraint = QueryContentTable::new("query");
        let mr = MdwtRecordTable::new("mr");

        let sr = SqlReader::builder(
            // select
            [SqlField {
                alias: None,
                table_alias: cm.alias,
                field_name: "*",
            }],
            // from
            // Chnot Thread Meta
            Joins::new(chin_sql::Froms::Table {
                table_name: cm.table(),
                alias: cm.alias,
            })
            .join_some(
                tag_constraint.sub_query_table(req.tags.clone(), req.get_spaces()),
                |v| JoinTable {
                    join_type: JoinType::InnerJoin,
                    table: Froms::SubQuery {
                        table: v.reader.into(),
                        alias: &tag_constraint.alias,
                    },
                    conds: [(tag_constraint.mdwt_otid(), cm.otid()).into()].into(),
                },
            )
            .join_some(
                QueryContentTable::sub_query_table(req.query.as_deref()),
                |v| JoinTable {
                    join_type: JoinType::InnerJoin,
                    table: Froms::SubQuery {
                        table: v.reader.into(),
                        alias: query_constraint.alias,
                    },
                    conds: [(query_constraint.chnot_otid(), cm.otid()).into()].into(),
                },
            )
            .join(JoinTable {
                join_type: JoinType::LeftJoin,
                table: Froms::Table {
                    table_name: mr.table(),
                    alias: mr.alias,
                },
                conds: [(mr.otid(), cm.otid()).into()].into(),
            })
            .into(),
        )
        .wheres(Wheres::and([cm.kspace().v_in(req.get_spaces())]))
        .order_by([
            OrderBy::Desc(cm.pin_tid().twn()),
            OrderBy::Desc(cm.otid().twn()),
        ])
        .build();

        self.conn()
            .await?
            .page_read(
                sr,
                LimitOffset {
                    limit: req.page_size,
                    offset: req.start_index.into(),
                },
                |row| {
                    Ok(ChnotSearchRspSingle {
                        meta: ChnotMeta {
                            otid: row.try_get(ChnotMeta::OTID)?,
                            kspace: row.try_get(ChnotMeta::KSPACE)?,
                            pin_tid: row.try_get(ChnotMeta::PIN_TID)?,
                            archive_tid: row.try_get(ChnotMeta::ARCHIVE_TID)?,
                            tid: row.try_get(ChnotMeta::TID)?,
                            kind: row.try_get(ChnotMeta::KIND)?,
                        },
                        preview_text: row.try_get("cont")?,
                    })
                },
            )
            .await
    }
}

impl TryFrom<&KDbRow> for ChnotThreadMeta {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let chnot = ChnotThreadMeta {
            otid: value.try_get(ChnotThreadMeta::OTID)?,
            kspace: value.try_get(ChnotThreadMeta::KSPACE)?,
            archive_tid: value.try_get(ChnotThreadMeta::ARCHIVE_TID)?,
            pin_tid: value.try_get(ChnotThreadMeta::PIN_TID)?,
            tid: value.try_get(ChnotThreadMeta::TID)?,
        };
        Ok(chnot)
    }
}
