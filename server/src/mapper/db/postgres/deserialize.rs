use chin_tools::AResult;
use tokio_postgres::Row;

use crate::{
    mapper::DeserializeMapper,
    model::db::{
        chnot::*,
        kv::KV,
        llmchat::*,
        namespace::*,
        resource::{InlineResource, Resource},
    },
};

impl DeserializeMapper for Row {
    fn to_chnot_meta(self) -> AResult<ChnotMetadata> {
        let chnot = ChnotMetadata {
            id: self.try_get(ChnotMetadata::field_id())?,
            namespace: self.try_get(ChnotMetadata::field_namespace())?,
            kind: self.try_get(ChnotMetadata::field_kind())?,
            pin_time: self.try_get(ChnotMetadata::field_pin_time())?,
            delete_time: self.try_get(ChnotMetadata::field_delete_time())?,
            update_time: self.try_get(ChnotMetadata::field_update_time())?,
            insert_time: self.try_get(ChnotMetadata::field_insert_time())?,
            archive_time: self.try_get(ChnotMetadata::field_archive_time())?,
        };
        Ok(chnot)
    }

    fn to_chnot_record(self) -> AResult<ChnotRecord> {
        let chnot = ChnotRecord {
            id: self.try_get("id")?,
            meta_id: self.try_get("meta_id")?,
            content: self.try_get("content")?,
            omit_time: self.try_get("omit_time")?,
            insert_time: self.try_get("insert_time")?,
        };
        Ok(chnot)
    }

    fn to_llmchat_bot(self) -> AResult<LLMChatBot> {
        let obj = LLMChatBot {
            id: self.try_get("id")?,
            insert_time: self.try_get("insert_time")?,
            delete_time: self.try_get("delete_time")?,
            name: self.try_get("name")?,
            body: self.try_get("body")?,
            update_time: self.try_get("update_time")?,
            svg_logo: self.try_get("svg_logo")?,
        };
        Ok(obj)
    }

    fn to_llmchat_template(self) -> AResult<LLMChatTemplate> {
        let obj = LLMChatTemplate {
            id: self.try_get("id")?,
            insert_time: self.try_get("insert_time")?,
            delete_time: self.try_get("delete_time")?,
            update_time: self.try_get("update_time")?,
            name: self.try_get("name")?,
            prompt: self.try_get("prompt")?,
            svg_logo: self.try_get("svg_logo")?,
        };
        Ok(obj)
    }

    fn to_llmchat_session(self) -> AResult<LLMChatSession> {
        let obj = LLMChatSession {
            id: self.try_get("id")?,
            insert_time: self.try_get("insert_time")?,
            bot_id: self.try_get("bot_id")?,
            template_id: self.try_get("template_id")?,
            title: self.try_get("title")?,
            namespace: self.try_get("namespace")?,
            delete_time: self.try_get("delete_time")?,
            update_time: self.try_get("update_time")?,
        };
        Ok(obj)
    }

    fn to_llmchat_record(self) -> AResult<LLMChatRecord> {
        let obj = LLMChatRecord {
            id: self.try_get("id")?,
            insert_time: self.try_get("insert_time")?,
            session_id: self.try_get("session_id")?,
            pre_record_id: self.try_get("pre_record_id")?,
            content: self.try_get("content")?,
            role: self.try_get("role")?,
            role_id: self.try_get("role_id")?,
        };
        Ok(obj)
    }

    fn to_namespace_record(self) -> AResult<NamespaceRecord> {
        let obj = NamespaceRecord {
            id: self.try_get("id")?,
            insert_time: self.try_get("insert_time")?,
            name: self.try_get("name")?,
            delete_time: self.try_get("delete_time")?,
            update_time: self.try_get("update_time")?,
        };
        Ok(obj)
    }

    fn to_namespace_relation(self) -> AResult<NamespaceRelation> {
        let obj = NamespaceRelation {
            id: self.try_get("id")?,
            insert_time: self.try_get("insert_time")?,
            delete_time: self.try_get("delete_time")?,
            update_time: self.try_get("update_time")?,
            sub_id: self.try_get("sub_id")?,
            parent_id: self.try_get("parent_id")?,
        };
        Ok(obj)
    }

    fn to_resource(self) -> AResult<Resource> {
        let obj = Resource {
            id: self.try_get("id")?,
            insert_time: self.try_get("insert_time")?,
            delete_time: self.try_get("delete_time")?,
            namespace: self.try_get("namespace")?,
            ori_filename: self.try_get("ori_filename")?,
            content_type: self.try_get("content_type")?,
        };
        Ok(obj)
    }

    fn to_kv(self) -> AResult<KV> {
        let obj = KV {
            insert_time: self.try_get("insert_time")?,
            key: self.try_get("key")?,
            value: self.try_get("value")?,
            update_time: self.try_get("update_time")?,
        };
        Ok(obj)
    }

    fn to_chnot_tag(self) -> AResult<ChnotTag> {
        let obj = ChnotTag {
            id: self.try_get("id")?,
            namespace: self.try_get("namespace")?,
            tag: self.try_get("tag")?,
            chnot_meta_id: self.try_get("chnot_meta_id")?,
            insert_time: self.try_get("insert_time")?,
            category: ChnotTagType::Common,
        };
        Ok(obj)
    }

    fn to_inline_resource(self) -> AResult<crate::model::db::resource::InlineResource> {
        let obj = InlineResource {
            id: self.try_get(InlineResource::field_id())?,
            name: self.try_get(InlineResource::field_name())?,
            content: self.try_get(InlineResource::field_content())?,
            content_type: self.try_get(InlineResource::field_content_type())?,
            delete_time: self.try_get(InlineResource::field_delete_time())?,
            insert_time: self.try_get(InlineResource::field_insert_time())?,
        };
        Ok(obj)
    }
}
