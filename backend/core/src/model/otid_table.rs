use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Sequence)]
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
