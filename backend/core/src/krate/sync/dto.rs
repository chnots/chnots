use chin_sql::time_type::TID;
use serde::{de, Deserialize, Deserializer, Serialize};

use crate::mapper::TheSameKey;

#[derive(Debug, Clone, Copy, Serialize)]
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
    KTabDataDate,
    KTabDataDecimal,
    KTabDataText,
    KFileMeta // inline k file is a specifal type file, so we sync it with kfilemeta
}

impl TryFrom<&str> for SyncTableEnum {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let c = match value.to_lowercase().as_str() {
            "chnot_record" => SyncTableEnum::ChnotRecord,
            "chnot_meta" => SyncTableEnum::ChnotMetadata,
            "chnot_kind_rel" => SyncTableEnum::ChnotKindRel,
            "chnot_tag" => SyncTableEnum::ChnotTag,
            "llm_chat_bot" => SyncTableEnum::LLMChatBot,
            "llm_chat_record" => SyncTableEnum::LLMChatRecord,
            "llm_chat_template" => SyncTableEnum::LLMChatTemplate,
            "llm_chat_session" => SyncTableEnum::LLMChatSession,
            "kkv" => SyncTableEnum::KKV,
            "k_tab_meta" => SyncTableEnum::KTabMeta,
            "k_tab_data_date" => SyncTableEnum::KTabDataDate,
            "k_tab_data_decimal" => SyncTableEnum::KTabDataDecimal,
            "k_tab_data_text" => SyncTableEnum::KTabDataText,
            "k_file_meta" => SyncTableEnum::KFileMeta,
            _ => Err(anyhow::anyhow!("unable to deser from string {}", value))?
        };

        Ok(c)
    }
}

impl<'de> Deserialize<'de> for SyncTableEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        let c = SyncTableEnum::try_from(s.as_str()).map_err(|err| de::Error::custom(err.to_string()))?;
        Ok(c)
    }
}

pub struct SyncShakeReq {
    pub client_id: String,
    pub client_app_version: String,
    pub server_id: String,
}

pub struct SyncShakeRsp {
    pub stop: bool,
    pub last_sync_time: Option<TID>,
}

pub struct SyncFetchSameKeyReq {
    pub sync_id: String,
    pub table_name: SyncTableEnum,
    pub start_tid_ex: String,
}

pub struct SyncFetchSameKeyRsp {
    pub same_keys: Vec<TheSameKey>,
}

pub struct SyncFetchAbsentReq {
    pub table_name: SyncTableEnum,
    pub tids: Vec<TID>,
}

pub struct SyncFetchAbsentRsp<T: Serialize> {
    pub records: Vec<T>,
}

pub(crate) enum FetchDataType {
    RangePage {
        start_ex: TID,
        end_in: TID,
        page_size: usize,
    },
    Tids (Vec<TID>)
}
