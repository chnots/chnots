use std::{marker::PhantomData, ops::Deref};

use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Sequence)]
pub enum OtidTableEnum {
    MdwtRecord,
    ChnotThreadMeta,
    ChnotThreadOrder,
    MdwtTag,
    ChnotMeta,
    MdwtToent,
    LLMChatBot,
    LLMChatRecord,
    LLMChatTemplate,
    LLMChatSession,
    KSpace,
    #[allow(clippy::upper_case_acronyms)]
    KKV,
    KTabMeta,
    KTabCellDate,
    KTabCellDecimal,
    KTabCellText,
    KFileMeta, // inline k file is a specifal type file, so we sync it with kfilemeta
    GraphMeta, // graph data is a specifal type
}

#[macro_export]
macro_rules! otid_enum_to_generic {
    ($table_type:expr, $invoke:ident) => {{
        use $crate::model::otid_table::OtidTableEnum;
        match $table_type {
            OtidTableEnum::MdwtRecord => $invoke! {crate::krate::mdwt::MdwtRecord},
            OtidTableEnum::ChnotThreadMeta => $invoke! {crate::krate::chnot::ChnotThreadMeta},
            OtidTableEnum::MdwtTag => $invoke! {crate::krate::mdwt::MdwtTag},
            OtidTableEnum::LLMChatBot => $invoke! {crate::krate::llmchat::LLMChatBot},
            OtidTableEnum::LLMChatRecord => $invoke! {crate::krate::llmchat::LLMChatRecord},
            OtidTableEnum::LLMChatTemplate => $invoke! {crate::krate::llmchat::LLMChatTemplate},
            OtidTableEnum::LLMChatSession => $invoke! {crate::krate::llmchat::LLMChatSession},
            OtidTableEnum::KKV => $invoke! {crate::krate::kkv::KKV},
            OtidTableEnum::KTabMeta => $invoke! {crate::krate::ktab::KTabMeta},
            OtidTableEnum::KTabCellDate => $invoke! {crate::krate::ktab::KTabCellDate},
            OtidTableEnum::KTabCellDecimal => $invoke! {crate::krate::ktab::KTabCellDecimal},
            OtidTableEnum::KTabCellText => $invoke! {crate::krate::ktab::KTabCellText},
            OtidTableEnum::KFileMeta => $invoke! {crate::krate::ktab::KTabMeta},
            OtidTableEnum::KSpace => $invoke! {crate::krate::kspace::KSpace},
            OtidTableEnum::ChnotMeta => $invoke! {crate::krate::chnot::ChnotMeta},
            OtidTableEnum::MdwtToent => $invoke! {crate::krate::mdwt::MdwtRecord},
            OtidTableEnum::ChnotThreadOrder => $invoke! {crate::krate::chnot::ChnotThreadOrder},
            OtidTableEnum::GraphMeta => $invoke! {crate::krate::graph::GraphMeta},
        }
    }};
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OtidWithEnum<E> {
    pub(crate) table_type: OtidTableEnum,
    pub(crate) dto: E,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct OtidWithGeneric<E, T> {
    pub(crate) table_type: PhantomData<T>,
    pub(crate) dto: E,
}

impl<E, T> Deref for OtidWithGeneric<E, T> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.dto
    }
}
