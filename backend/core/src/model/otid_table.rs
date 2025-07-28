use chin_tools::AResult;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::{
    krate::{
        chnot::{ChnotKindRel, ChnotMetadata, ChnotRecord, ChnotTag},
        kfile::KFileMeta,
        kkv::KKV,
        ktab::{KTabCellDate, KTabCellDecimal, KTabCellText, KTabMeta},
        llmchat::{LLMChatBot, LLMChatRecord, LLMChatSession, LLMChatTemplate},
    },
    mapper::db::{HistCreateSql, KDbRow}, model::KOtidSupport,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter)]
pub enum OtidTableEnum {
    ChnotRecord,
    ChnotMetadata,
    ChnotKindRel,
    ChnotTag,
    LLMChatBot,
    LLMChatRecord,
    LLMChatTemplate,
    LLMChatSession,
    KSpace,
    KKV,
    KTabMeta,
    KTabCellDate,
    KTabCellDecimal,
    KTabCellText,
    KFileMeta, // inline k file is a specifal type file, so we sync it with kfilemeta
}

impl OtidTableEnum {
    pub fn to_table_name(self, hist: bool) -> &'static str {
        match self {
            OtidTableEnum::ChnotRecord => ChnotRecord::table_name(hist),
            OtidTableEnum::ChnotMetadata => ChnotMetadata::table_name(hist),
            OtidTableEnum::ChnotKindRel => ChnotKindRel::table_name(hist),
            OtidTableEnum::ChnotTag => ChnotTag::table_name(hist),
            OtidTableEnum::LLMChatBot => LLMChatBot::table_name(hist),
            OtidTableEnum::LLMChatRecord => LLMChatRecord::table_name(hist),
            OtidTableEnum::LLMChatTemplate => LLMChatTemplate::table_name(hist),
            OtidTableEnum::LLMChatSession => LLMChatSession::table_name(hist),
            OtidTableEnum::KKV => KKV::table_name(hist),
            OtidTableEnum::KTabMeta => crate::krate::ktab::KTabMeta::table_name(hist),
            OtidTableEnum::KTabCellDate => KTabCellDate::table_name(hist),
            OtidTableEnum::KTabCellDecimal => KTabCellDecimal::table_name(hist),
            OtidTableEnum::KTabCellText => KTabCellText::table_name(hist),
            OtidTableEnum::KFileMeta => crate::krate::kfile::KFileMeta::table_name(hist),
            OtidTableEnum::KSpace => crate::krate::kspace::KSpace::table_name(hist),
        }
    }
}

macro_rules! otid_call_with_ger {
    ($ote:expr, $obj:ident.$method:ident($($arg:expr),*)) => {
        match $ote {
            OtidTableEnum::ChnotRecord => $obj.$method::<ChnotRecord>($($arg),*),
            OtidTableEnum::ChnotMetadata => $obj.$method::<ChnotMetadata>($($arg),*),
            OtidTableEnum::ChnotKindRel => $obj.$method::<ChnotKindRel>($($arg),*),
            OtidTableEnum::ChnotTag => $obj.$method::<ChnotTag>($($arg),*),
            OtidTableEnum::LLMChatBot => $obj.$method::<LLMChatBot>($($arg),*),
            OtidTableEnum::LLMChatRecord => $obj.$method::<LLMChatRecord>($($arg),*),
            OtidTableEnum::LLMChatTemplate => $obj.$method::<LLMChatTemplate>($($arg),*),
            OtidTableEnum::LLMChatSession => $obj.$method::<LLMChatSession>($($arg),*),
            OtidTableEnum::KKV => $obj.$method::<KKV>($($arg),*),
            OtidTableEnum::KTabMeta => $obj.$method::<crate::krate::ktab::KTabMeta>($($arg),*),
            OtidTableEnum::KTabCellDate => $obj.$method::<KTabCellDate>($($arg),*),
            OtidTableEnum::KTabCellDecimal => $obj.$method::<KTabCellDecimal>($($arg),*),
            OtidTableEnum::KTabCellText => $obj.$method::<KTabCellText>($($arg),*),
            OtidTableEnum::KFileMeta => $obj.$method::<crate::krate::kfile::KFileMeta>($($arg),*),
        }
    };
}
