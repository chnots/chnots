use chin_sql::time_type::TID;
use chin_tools::SharedStr;
use serde::{Deserialize, Deserializer, Serialize, de};
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

impl TryFrom<&str> for SyncTableEnum {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
}

impl ToString for SyncTableEnum {
    fn to_string(&self) -> String {
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncShakeReq {
    pub client_id: SharedStr,
    pub app_version: String,
    pub table_name: SyncTableEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncShakeRspEnum {
    NotSameVersion(String),
    BeginSync { sync_time: TID },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncShakeRsp {
    pub instance_id: String,
    pub data: SyncShakeRspEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncFetchDataReq {
    pub table_name: SyncTableEnum,
    pub fetch_data: FetchDataType,
    pub hist: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncFetchDataRsp<T: Serialize> {
    pub records: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum FetchDataType {
    RangePage {
        start_ex: TID,
        end_in: TID,
        page_size: usize,
    },
    Tids(Vec<TID>),
}
