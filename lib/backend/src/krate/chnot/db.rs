use std::collections::HashMap;

use super::mapper::ChnotMapper;
use super::*;
use crate::krate::llmchat::LLMChatRecordTable;
use crate::krate::mdwt::db::MdwtOtidInTags;
use crate::krate::mdwt::{MdwtRecord, MdwtRecordTable};
use crate::mapper::Curd;
use crate::mapper::db::helper::{Ddls, print_ddls};
use crate::mapper::db::{
    HistCreateSql, KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRowBehavier, KDbTransactionBehaiver,
    PageReader,
};
use crate::model::dto::{KReq, PageRsp};
use crate::model::{KSerde, OtidTableSupport};
use crate::util::string_util::StringUtils;
use chin_sql::str_type::Text;
use chin_sql::time_type::TID;
use chin_sql::{
    Froms, GenerateTableSchema, LimitOffset, SqlField, SqlFieldTrait, SqlTable, Wheres,
};
use chin_sql::{ILikeType, JoinTable, JoinType, Joins, OrderBy, SqlBuilder, SqlReader};
use chin_tools::{AResult, EResult};

#[allow(dead_code)]
#[derive(Debug, Clone, GenerateTableSchema)]
struct QueryContent {
    #[gts_type = "i64"]
    chnot_otid: TID,
    content: Text,
}

impl<'a> QueryContentTable<'a> {
    fn sub_query_table(query: Option<&'a str>) -> Option<SqlReader<'a>> {
        let mr = MdwtRecordTable::new("mr");
        let lcr = LLMChatRecordTable::new("lcr");

        if let Some(query) = query.as_ref()
            && !query.is_empty()
        {
            let mdwt = SqlReader::read(
                (
                    mr.otid().with_alias(QueryContent::CHNOT_OTID),
                    mr.content().with_alias(QueryContent::CONTENT),
                ),
                &mr,
            )
            .wheres(mr.content().v_ilike(query, ILikeType::Fuzzy))
            .build();

            let llmchat = SqlReader::read(
                (
                    lcr.otid().with_alias(QueryContent::CHNOT_OTID),
                    lcr.content().with_alias(QueryContent::CONTENT),
                ),
                &lcr,
            )
            .wheres(lcr.content().v_ilike(query, ILikeType::Fuzzy))
            .build();

            Some(
                SqlReader::read(
                    vec![
                        SqlField {
                            alias: None,
                            inner: chin_sql::SqlFieldInner::Plain {
                                table_alias: "cont",
                                field_name: QueryContent::CHNOT_OTID,
                            },
                        },
                        SqlField {
                            alias: None,
                            inner: chin_sql::SqlFieldInner::Plain {
                                table_alias: "cont",
                                field_name: QueryContent::CONTENT,
                            },
                        },
                    ],
                    chin_sql::Froms::SubQuery {
                        table: SqlReader::Union(vec![llmchat.into(), mdwt.into()]).into(),
                        alias: "cont",
                    },
                )
                .build2(),
            )
        } else {
            None
        }
    }
}

impl ChnotMapper for KDb {
    async fn ensure_table_chnot(&self) -> EResult {
        print_ddls(
            Ddls::new()
                .with_ddls(ChnotThreadOrder::ddls())
                .with_ddls(ChnotMeta::ddls()),
            self,
        )
        .await
    }

    async fn chnot_thread_order_commit(
        &self,
        req: KReq<ChnotThreadOrderCommitReq>,
    ) -> AResult<ChnotThreadOrderCommitRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.transaction().await?;
        let ChnotThreadOrderCommitReq {
            thread_otid,
            orders,
            remove_others,
        } = req.body;

        let cto = ChnotThreadOrderTable::new("cto");
        let cto_otid = cto.otid().field_name();
        let korder_name = cto.korder().field_name();
        let closed_name = cto.closed().field_name();

        #[derive(Debug, PartialEq, Eq)]
        struct OtidAndOrder {
            otid: TID,
            korder: i64,
            closed: bool,
        }
        let saved_orders = tx
            .qry_list(
                SqlReader::read((cto.korder(), cto.otid(), cto.closed()), &cto)
                    .wheres(cto.thread_otid().v_eq(thread_otid))
                    .build(),
                |r| {
                    Ok(OtidAndOrder {
                        otid: r.try_get(cto_otid)?,
                        korder: r.try_get(korder_name)?,
                        closed: r.try_get(closed_name)?,
                    })
                },
            )
            .await?;

        let mut to_save_map: HashMap<TID, OtidAndOrder> = orders
            .into_iter()
            .enumerate()
            .map(|(index, data)| {
                (
                    data.otid,
                    OtidAndOrder {
                        otid: data.otid,
                        korder: index as i64,
                        closed: data.closed,
                    },
                )
            })
            .collect();
        let mut to_remove_list: Vec<TID> = vec![];
        saved_orders.into_iter().for_each(|saved_otid_and_order| {
            let to_save_otid_and_order = to_save_map.get(&saved_otid_and_order.otid);
            if to_save_otid_and_order.is_none_or(|data| data != &saved_otid_and_order) {
                to_remove_list.push(saved_otid_and_order.otid);
            } else {
                to_save_map.remove(&saved_otid_and_order.otid);
            }
        });

        if remove_others.unwrap_or_default() {
            tx.omit_rows::<ChnotThreadOrder>(Wheres::r#in(ChnotThreadOrder::OTID, to_remove_list))
                .await?;
        }

        let mut metas = vec![];

        for oao in to_save_map.values() {
            let rec = ChnotThreadOrder {
                otid: oao.otid,
                tid: TID::default(),
                thread_otid,
                korder: oao.korder,
                closed: oao.closed,
            };

            metas.push(rec.clone());

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

        let mut metas = conn
            .qry_list(
                SqlBuilder::read(
                    ChnotMeta::TABLE,
                    &[
                        format!("{}.*", ChnotMeta::TABLE).as_str(),
                        ChnotThreadOrder::CLOSED,
                    ],
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
                    req.otid,
                )]))
                .order_by([OrderBy::Asc(ChnotThreadOrder::KORDER.into())]),
                |row| {
                    Ok(ChnotThreadMetaFetchRspData {
                        meta: ChnotMeta::try_from(&row)?,
                        closed: row.try_get(ChnotThreadOrder::CLOSED)?,
                    })
                },
            )
            .await?;
        if req.include_hist.unwrap_or_default() {
            let metas_hist = conn
                .qry_list(
                    SqlBuilder::read(
                        ChnotMeta::TABLE,
                        &[
                            format!("{}.*", ChnotMeta::TABLE).as_str(),
                            ChnotThreadOrder::CLOSED,
                        ],
                    )
                    .seg(format!(
                        " left join {} on {}.{} = {}.{} ",
                        ChnotThreadOrder::table_name(true),
                        ChnotMeta::TABLE,
                        ChnotMeta::OTID,
                        ChnotThreadOrder::table_name(true),
                        ChnotThreadOrder::OTID
                    ))
                    .r#where(Wheres::and([Wheres::equal(
                        ChnotThreadOrder::THREAD_OTID.prefix_with_sep(ChnotThreadOrder::TABLE, "."),
                        req.otid,
                    )]))
                    .order_by([OrderBy::Asc(ChnotThreadOrder::KORDER.into())]),
                    |row| {
                        Ok(ChnotThreadMetaFetchRspData {
                            meta: ChnotMeta::try_from(&row)?,
                            closed: row.try_get(ChnotThreadOrder::CLOSED)?,
                        })
                    },
                )
                .await?;
            metas.extend(metas_hist)
        }

        Ok(ChnotThreadMetaFetchRsp {
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

            tx.omit_rows::<ChnotMeta>(rec.pkey()).await?;
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

    async fn chnot_search(
        &self,
        req: KReq<ChnotSearchReq>,
    ) -> AResult<PageRsp<ChnotSearchRspData>> {
        let cm = ChnotMetaTable::new("cm");
        let tag_constraint = MdwtOtidInTags::new("tag_constraint");
        let query_constraint = QueryContentTable::new("query");
        let mr = MdwtRecordTable::new("mr");
        let cto = ChnotThreadOrderTable::new("cto");
        // search every chnot
        let search_every_chnot = req.query.as_ref().is_some_and(|s| s.len() > 0)
            || req.tags.as_ref().is_some_and(|s| !s.is_empty())
            || !req.kinds.is_empty();

        let joins = Joins::new((&cm).into())
            .join_some(
                tag_constraint.sub_query_table(req.tags.clone(), req.get_spaces()),
                |v| JoinTable {
                    join_type: JoinType::InnerJoin,
                    table: Froms::SubQuery {
                        table: v.into(),
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
                        table: v.into(),
                        alias: query_constraint.alias,
                    },
                    conds: [(query_constraint.chnot_otid(), cm.otid()).into()].into(),
                },
            )
            .join(JoinTable {
                join_type: JoinType::LeftJoin,
                table: (&mr).into(),
                conds: [(mr.otid(), cm.otid()).into()].into(),
            })
            .join_if(
                !search_every_chnot,
                JoinTable {
                    join_type: JoinType::LeftJoin,
                    table: (&cto).into(),
                    conds: [(cto.otid(), cm.otid()).into()].into(),
                },
            );

        let where_clause = Wheres::and([
            cm.kspace().v_in(req.get_spaces()),
            Wheres::transform(req.kinds.clone(), |kinds| {
                if !kinds.is_empty() {
                    cm.kind().v_in(req.kinds.clone())
                } else {
                    Wheres::None
                }
            }),
            if !search_every_chnot {
                cto.otid().v_is_null()
            } else {
                Wheres::None
            },
            cm.archive_tid().v_is_null(),
        ]);

        let sr = SqlReader::read((cm.all_fields(), mr.content()), joins)
            .wheres(where_clause)
            .order_by([
                OrderBy::Desc(
                    format!(
                        "case when {} is null then 0 else {} end",
                        cm.pin_tid().twn(),
                        cm.pin_tid().twn()
                    )
                    .into(),
                ),
                OrderBy::Desc(cm.otid().twn()),
            ])
            .build2();

        self.conn()
            .await?
            .page_read(
                sr,
                LimitOffset {
                    limit: req.page_size,
                    offset: req.start_index.into(),
                },
                |row| {
                    Ok(ChnotSearchRspData {
                        meta: ChnotMeta {
                            otid: row.try_get(ChnotMeta::OTID)?,
                            kspace: row.try_get(ChnotMeta::KSPACE)?,
                            pin_tid: row.try_get(ChnotMeta::PIN_TID)?,
                            archive_tid: row.try_get(ChnotMeta::ARCHIVE_TID)?,
                            tid: row.try_get(ChnotMeta::TID)?,
                            kind: row.try_get(ChnotMeta::KIND)?,
                        },
                        title: row.try_get(MdwtRecord::CONTENT)?,
                    })
                },
            )
            .await
    }

    async fn chnot_thread_order_archive(
        &self,
        req: KReq<ChnotThreadOrderArchiveReq>,
    ) -> AResult<ChnotThreadOrderArchiveRsp> {
        let cto = ChnotThreadOrderTable::new("cto");
        let conn = self.conn().await?;
        conn.omit_rows::<ChnotThreadOrder>(Wheres::and([
            cto.thread_otid().v_eq(req.thread_otid),
            cto.otid().v_in(req.body.otids),
        ]))
        .await?;

        Ok(ChnotThreadOrderArchiveRsp {})
    }
}
