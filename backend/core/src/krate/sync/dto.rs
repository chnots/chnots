use std::fmt::Debug;

use chin_sql::time_type::TID;
use chin_tools::{AResult, SharedStr};
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::krate::{
    chnot::{ChnotKindRel, ChnotMetadata, ChnotRecord, ChnotTag},
    kkv::KKV,
    ktab::{KTabCellDate, KTabCellDecimal, KTabCellText},
    llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum SyncTableEnum {
    ChnotRecord,
    ChnotMetadata,
    ChnotKindRel,
    ChnotTag,
    LLMChatBot,
    LLMChatRecord,
    LLMChatTemplate,
    LLMChatSession,
    KKV,
    KTabMeta,
    KTabCellDate,
    KTabCellDecimal,
    KTabCellText,
    KFileMeta, // inline k file is a specifal type file, so we sync it with kfilemeta
}

impl SyncTableEnum {
    pub fn try_from_table_name(value: &str) -> AResult<Self> {
        let c = match value.to_lowercase().as_str() {
            ChnotRecord::TABLE => SyncTableEnum::ChnotRecord,
            ChnotMetadata::TABLE => SyncTableEnum::ChnotMetadata,
            ChnotKindRel::TABLE => SyncTableEnum::ChnotKindRel,
            ChnotTag::TABLE => SyncTableEnum::ChnotTag,
            LLMChatBot::TABLE => SyncTableEnum::LLMChatBot,
            LLMChatRecord::TABLE => SyncTableEnum::LLMChatRecord,
            LLMChatTemplate::TABLE => SyncTableEnum::LLMChatTemplate,
            LLMChatSession::TABLE => SyncTableEnum::LLMChatSession,
            KKV::TABLE => SyncTableEnum::KKV,
            crate::krate::ktab::KTabMeta::TABLE => SyncTableEnum::KTabMeta,
            KTabCellDate::TABLE => SyncTableEnum::KTabCellDate,
            KTabCellDecimal::TABLE => SyncTableEnum::KTabCellDecimal,
            KTabCellText::TABLE => SyncTableEnum::KTabCellText,
            crate::krate::kfile::KFileMeta::TABLE => SyncTableEnum::KFileMeta,
            _ => Err(anyhow::anyhow!("unable to deser from string {}", value))?,
        };

        Ok(c)
    }

    pub fn to_table_name(&self) -> String {
        match self {
            SyncTableEnum::ChnotRecord => ChnotRecord::TABLE.to_string(),
            SyncTableEnum::ChnotMetadata => ChnotMetadata::TABLE.to_string(),
            SyncTableEnum::ChnotKindRel => ChnotKindRel::TABLE.to_string(),
            SyncTableEnum::ChnotTag => ChnotTag::TABLE.to_string(),
            SyncTableEnum::LLMChatBot => LLMChatBot::TABLE.to_string(),
            SyncTableEnum::LLMChatRecord => LLMChatRecord::TABLE.to_string(),
            SyncTableEnum::LLMChatTemplate => LLMChatTemplate::TABLE.to_string(),
            SyncTableEnum::LLMChatSession => LLMChatSession::TABLE.to_string(),
            SyncTableEnum::KKV => KKV::TABLE.to_string(),
            SyncTableEnum::KTabMeta => crate::krate::ktab::KTabMeta::TABLE.to_string(),
            SyncTableEnum::KTabCellDate => KTabCellDate::TABLE.to_string(),
            SyncTableEnum::KTabCellDecimal => KTabCellDecimal::TABLE.to_string(),
            SyncTableEnum::KTabCellText => KTabCellText::TABLE.to_string(),
            SyncTableEnum::KFileMeta => crate::krate::kfile::KFileMeta::TABLE.to_string(),
        }
    }

    pub fn to_hist_table_name(&self) -> String {
        return self.to_table_name() + "_hist";
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncInfo {
    pub instance_id: SharedStr,
    pub start_ex: TID,
    pub end_in: TID,
    pub table: SyncTableEnum,
}

impl SyncInfo {
    pub fn to_table_name(&self) -> String {
        format!(
            "{}_{}_{}_{}",
            self.instance_id.as_str(),
            self.table.to_table_name(),
            self.start_ex,
            self.end_in
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncShakeReq {
    pub instance_id: SharedStr,
    pub db_version: String,
    pub table_name: SyncTableEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncShakeRspEnum {
    NotSameVersion(String),
    SameClient,
    BeginSync { sync_time: TID },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncShakeRsp {
    pub instance_id: SharedStr,
    pub data: SyncShakeRspEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncFetchTIDReq {
    pub sync_info: SyncInfo,
    pub page_size: usize,
    pub hist: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct FetchTIDReq {
    pub start_ex: TID,
    pub end_in: TID,
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncFetchTIDRsp {
    pub data: Vec<TID>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDataReq<E> {
    cmds: Vec<SyncDataReqEnum<E>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncDataReqEnum<E> {
    Omit(TID),
    LeftCur(TID),
    LeftHist(TID),
    RightCur(E),
    RightHist(E),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDataRsp<E> {
    cmds: Vec<SyncDataRspEnum<E>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncDataRspEnum<E> {
    Cur(E),
    Hist(E),
}
