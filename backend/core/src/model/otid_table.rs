use serde::{Deserialize, Serialize};
use strum::EnumIter;

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
    #[allow(clippy::upper_case_acronyms)]
    KKV,
    KTabMeta,
    KTabCellDate,
    KTabCellDecimal,
    KTabCellText,
    KFileMeta, // inline k file is a specifal type file, so we sync it with kfilemeta
}
