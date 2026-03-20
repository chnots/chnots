use std::borrow::Cow;

use super::*;
use crate::krate::mdwt::mapper::MdwtMapper;
use crate::krate::mdwt::parser::MdwtParser;
use crate::krate::toent::ToentDefiCommitReq;
use crate::krate::toent::logic::EventBuilder;
use crate::krate::toent::logic::todoevent::TodoEvent;
use crate::krate::toent::po::TimeEventField;
use crate::mapper::db::helper::{Ddls, print_ddls};
use crate::mapper::db::{
    HistCreateSql, KDb, KDbBehaiver, KDbExecutorBehaiver, KDbRow, KDbRowBehavier,
    KDbTransactionBehaiver, KDbTx,
};
use crate::model::dto::KReq;
use crate::util::result_util::UnwrapOr;
use chin_sql::str_type::Varchar;
use chin_sql::time_type::TID;
use chin_sql::{
    ChinSqlError, GroupBy, Having, JoinTable, Joins, SqlBuilder, SqlField, SqlReader, SqlTable,
    SqlTypedField,
};
use chin_sql::{LimitOffset, Wheres};
use chin_tools::{AResult, EResult};
use chrono::TimeDelta;
use itertools::Itertools;
use log::info;
use serde::Serialize;

const UNTAGGED_TAG: &str = "<NON>";

impl<'a> KDbTx<'a> {
    pub(super) async fn chnot_tag_update_single_chnot(
        &self,
        req: MdwtTagUpdateReq,
        chnot_parser: &MdwtParser<'_>,
    ) -> EResult {
        let MdwtTagUpdateReq {
            content: _,
            mdwt_otid,
        } = req;

        let tags: Result<Vec<Varchar<800>>, ChinSqlError> = chnot_parser
            .get_all_tags()
            .iter()
            .map(|t| t.to_string().try_into())
            .collect();
        let mut tags = tags?;
        self.omit_rows::<MdwtTag>(Wheres::and([
            Wheres::equal(MdwtTag::MDWT_OTID, mdwt_otid),
            if tags.is_empty() {
                Wheres::None
            } else {
                Wheres::not(Wheres::r#in(MdwtTag::TAG, tags.clone()))
            },
        ]))
        .await?;
        let executor = self.as_executor();
        if tags.is_empty() {
            tags.push(UNTAGGED_TAG.try_into()?);
        }

        for tag in tags {
            executor
                .exec(
                    MdwtTag {
                        tid: TID::now(),
                        tag: tag.to_owned(),
                        mdwt_otid,
                    }
                    .to_sql_inserter()
                    .on_conflict(chin_sql::OnConflict::Ignore),
                )
                .await?;
        }

        Ok(())
    }

    async fn overwrite_mdwt_record(&self, block: MdwtCommitReqData) -> EResult {
        struct OldInfo {
            tid: TID,
            content: String,
        }
        fn to_old_info(row: KDbRow) -> AResult<OldInfo> {
            Ok(OldInfo {
                tid: row.try_get(MdwtRecord::OTID)?,
                content: row.try_get(MdwtRecord::CONTENT)?,
            })
        }

        // TODO: parse content and backlinks
        let _mdwt_parser = MdwtParser::new(block.content.as_str());
        let rec_tid: TID = TID::now();

        // Query for existing record
        let query_old_rec =
            SqlBuilder::read(MdwtRecord::TABLE, &[MdwtRecord::OTID, MdwtRecord::CONTENT])
                .r#where(Wheres::and([Wheres::equal(MdwtRecord::OTID, block.otid)]));

        // Get old record info
        let archor = if let Some(OldInfo {
            tid: old_id,
            content: old_cont,
        }) = self.qry_opt(query_old_rec, to_old_info).await?
        {
            let time_delta = rec_tid
                .as_utc()
                .signed_duration_since(old_id.as_utc())
                .abs();

            if old_cont.len() == block.content.as_str().len() && old_cont == block.content.as_str()
            {
                return Ok(());
            }

            (textdistance::str::sift4_simple(&old_cont, block.content.as_str()) >= 60
                && time_delta > TimeDelta::minutes(3))
                || time_delta > TimeDelta::hours(1)
        } else {
            false
        };

        let rec = MdwtRecord {
            otid: block.otid,
            tid: rec_tid,
            todo_event: None,
            content: block.content,
            archor,
        };

        self.po_otid_commit([rec]).await?;

        Ok(())
    }

    pub(super) async fn mdwt_commit(&self, req: KReq<MdwtCommitReq>) -> AResult<MdwtCommitRsp> {
        let MdwtCommitReq { mdwt } = req.body;
        let title = mdwt.content.as_str().split('\n').take(1).join("");
        let mdwt_parser = MdwtParser::new(mdwt.content.as_str());
        let todo_event = mdwt_parser.get_outer_todo_event();
        let time_events = mdwt_parser.get_outer_time_events();
        let otid = mdwt.otid;

        self.overwrite_mdwt_record(mdwt).await?;
        self.toent_defi_commit(&ToentDefiCommitReq {
            otid,
            todo_event,
            time_event_field: TimeEventField {
                time_events: time_events.into_iter().collect(),
            }
            .into(),
        })
        .await?;

        Ok(MdwtCommitRsp {
            todo_event,
            title: title.into(),
        })
    }
}

pub(crate) struct MdwtOtidInTags<'a> {
    pub alias: Cow<'a, str>,
    mt: MdwtTagTable<'a>,
}

impl<'a> MdwtOtidInTags<'a> {
    pub fn new(alias: &'a str) -> Self {
        let mt = MdwtTagTable::new("mt");
        Self {
            alias: alias.into(),
            mt,
        }
    }

    pub fn mdwt_otid(&'a self) -> SqlTypedField<'a, TID> {
        SqlTypedField::new(&self.alias, MdwtTag::MDWT_OTID)
    }

    pub fn sub_query_table(&'a self, tags: Option<MdwtTagSearchType>) -> Option<SqlReader<'a>> {
        let tags = tags.map(|s| match s {
            MdwtTagSearchType::Inset(items) => items,
        })?;
        let len = if !tags.is_empty() {
            tags.len()
        } else {
            return None;
        };

        let reader = SqlReader::read(self.mt.mdwt_otid(), &self.mt)
            .wheres(Wheres::and([self.mt.tag().v_in(
                tags.iter()
                    .map(Varchar::<800>::limit)
                    .collect::<Vec<Varchar<800>>>(),
            )]))
            .group_by(GroupBy::Plain([MdwtTag::MDWT_OTID.into()].into()))
            .having(Having::Custom(
                format!("COUNT(DISTINCT {}) = {}", MdwtTag::TAG, len).into(),
            ))
            .build();

        Some(reader.into())
    }
}

impl KDb {
    async fn chnot_tag_query_inner<F, T>(
        &self,
        req: KReq<MdwtTagListReq>,
        mapper: F,
        name_only: bool,
    ) -> AResult<MdwtTagListRsp<T>>
    where
        F: Fn(KDbRow) -> AResult<T> + Send + 'static,
        T: Serialize + Clone + Send + 'static + AsRef<str>,
    {
        let field_name = if name_only { "distinct tag" } else { "*" };
        let otids = MdwtOtidInTags::new("otids");
        let mt = MdwtTagTable::new("tags");

        let sql = SqlReader::read(
            SqlField {
                alias: None,
                inner: chin_sql::SqlFieldInner::Raw {
                    expr: field_name.into(),
                },
            },
            Joins::new((&mt).into()).join_some(otids.sub_query_table(req.tags.clone()), |v| {
                JoinTable {
                    join_type: chin_sql::JoinType::RightJoin,
                    table: chin_sql::Froms::SubQuery {
                        table: v.into(),
                        alias: &otids.alias,
                    },
                    conds: [(otids.mdwt_otid(), mt.mdwt_otid()).into()].into(),
                }
            }),
        )
        .wheres(Wheres::and([Wheres::if_some(req.query.clone(), |v| {
            mt.tag().v_ilike(v, chin_sql::ILikeType::Fuzzy)
        })]))
        .limit(LimitOffset::new(req.page_size).offset(req.start_index))
        .build();

        let data = self.conn().await?.qry_list(sql, mapper).await?;

        Ok(MdwtTagListRsp {
            data,
            start_index: req.start_index,
        })
    }
}

impl MdwtMapper for KDb {
    async fn mdwt_commit(&self, req: KReq<MdwtCommitReq>) -> AResult<MdwtCommitRsp> {
        let mut conn = self.conn().await?;
        let tx = conn.transaction().await?;
        let rsp = tx.mdwt_commit(req).await;

        if rsp.is_ok() {
            tx.cmt().await?;
        } else {
            tx.rbk().await?;
        }

        rsp
    }

    async fn mdwt_tag_list(&self, req: KReq<MdwtTagListReq>) -> AResult<MdwtTagListRsp<MdwtTag>> {
        self.chnot_tag_query_inner(req, |e| (&e).try_into(), false)
            .await
    }

    async fn mdwt_tag_name_list(
        &self,
        req: KReq<MdwtTagListReq>,
    ) -> AResult<MdwtTagListRsp<String>> {
        info!("{req:#?}");
        let remove_params = req.remove_params.default_true();
        let input_tag = req.body.tags.as_ref().map_or(vec![], |c| match c {
            MdwtTagSearchType::Inset(items) => items.to_vec(),
        });
        let mut result: MdwtTagListRsp<String> = self
            .chnot_tag_query_inner(req, |e| e.try_get(MdwtTag::TAG), true)
            .await?;

        result.data = result
            .data
            .into_iter()
            .unique()
            .filter(|e| {
                if remove_params {
                    !input_tag.contains(e)
                } else {
                    true
                }
            })
            .collect();

        Ok(result)
    }

    async fn mdwt_tag_refresh(&self) -> EResult {
        let mr = MdwtRecordTable::new("mr");
        let get_all = SqlReader::read(mr.all_fields(), Joins::new((&mr).into())).build();

        let mut conn = self.conn().await?;
        let chnots = conn
            .qry_list(get_all, move |e| {
                Ok(MdwtTagUpdateReq {
                    content: e.try_get(MdwtRecord::CONTENT)?,
                    mdwt_otid: e.try_get(MdwtRecord::OTID)?,
                })
            })
            .await?;

        let tx = conn.transaction().await?;
        for one in chnots {
            let chnot_parser = parser::MdwtParser::new(one.content.as_str());
            tx.chnot_tag_update_single_chnot(one.clone(), &chnot_parser)
                .await?;
        }
        tx.cmt().await?;

        Ok(())
    }

    async fn mdwt_list(&self, req: KReq<MdwtRecordsReq>) -> AResult<MdwtRecordsRsp> {
        let recs = self
            .conn()
            .await?
            .qry_list(
                SqlBuilder::read_all(MdwtRecord::TABLE).r#where(Wheres::or([Wheres::r#in(
                    MdwtRecord::OTID,
                    req.body.mdwt_otids,
                )])),
                |row| MdwtRecord::try_from(&row),
            )
            .await?;

        Ok(MdwtRecordsRsp {
            mdwt_map: recs.into_iter().map(|r| (r.otid, r)).collect(),
        })
    }

    async fn ensure_table_mdwt(&self) -> EResult {
        print_ddls(
            Ddls::new()
                .with_ddls(MdwtRecord::ddls())
                .with_ddls(MdwtTag::ddls()),
            self,
        )
        .await
    }
}

impl TryFrom<&KDbRow> for MdwtRecord {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let chnot = Self {
            content: value.try_get(Self::CONTENT)?,
            tid: value.try_get(Self::TID)?,
            archor: value.try_get(Self::ARCHOR)?,
            todo_event: {
                let opt: Option<String> = value.try_get(Self::TODO_EVENT)?;
                match opt {
                    Some(opt) => Some(TodoEvent::try_from_standrd_str(opt.as_str())?),
                    None => None,
                }
            },
            otid: value.try_get(Self::OTID)?,
        };
        Ok(chnot)
    }
}

impl TryFrom<&KDbRow> for MdwtTag {
    type Error = anyhow::Error;

    fn try_from(value: &KDbRow) -> Result<Self, Self::Error> {
        let obj = MdwtTag {
            tid: value.try_get(MdwtTag::TID)?,
            tag: value.try_get(MdwtTag::TAG)?,
            mdwt_otid: value.try_get(MdwtTag::MDWT_OTID)?,
        };
        Ok(obj)
    }
}
