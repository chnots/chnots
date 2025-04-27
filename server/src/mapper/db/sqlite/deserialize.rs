use chin_sql::DateFixed;
use chin_tools::AResult;
use chrono::{DateTime, FixedOffset};
use deadpool_sqlite::rusqlite::Row;

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

fn get_df<'a>(row: &Row<'a>, key: &str) -> AResult<DateTime<FixedOffset>> {
    Ok(row.get::<&str, DateFixed>(key).map(|e| e.fixed_offset())?)
}

fn get_op_df<'a>(row: &Row<'a>, key: &str) -> AResult<Option<DateTime<FixedOffset>>> {
    Ok(row
        .get::<&str, Option<DateFixed>>(key)
        .map(|e| e.map(|e1| e1.fixed_offset()))?)
}

impl<'a> DeserializeMapper for &'a Row<'a> {
    fn to_chnot_meta(self) -> AResult<ChnotMetadata> {
        let chnot = ChnotMetadata {
            id: self.get(ChnotMetadata::field_id())?,
            namespace: self.get(ChnotMetadata::field_namespace())?,
            kind: self.get(ChnotMetadata::field_kind())?,
            pin_time: get_op_df(&self, ChnotMetadata::field_pin_time())?,
            delete_time: get_op_df(&self, ChnotMetadata::field_delete_time())?,
            update_time: get_op_df(&self, ChnotMetadata::field_update_time())?,
            insert_time: get_df(&self, ChnotMetadata::field_insert_time())?,
            archive_time: get_op_df(&self, ChnotMetadata::field_archive_time())?,
        };
        Ok(chnot)
    }

    fn to_chnot_record(self) -> AResult<ChnotRecord> {
        let chnot = ChnotRecord {
            id: self.get("id")?,
            meta_id: self.get("meta_id")?,
            content: self.get("content")?,
            omit_time: get_op_df(&self, "omit_time")?,
            insert_time: get_df(&self, "insert_time")?,
        };
        Ok(chnot)
    }

    fn to_llmchat_bot(self) -> AResult<LLMChatBot> {
        let obj = LLMChatBot {
            id: self.get("id")?,
            insert_time: get_df(&self, "insert_time")?,
            delete_time: get_op_df(&self, "delete_time")?,
            name: self.get("name")?,
            body: self.get("body")?,
            update_time: get_op_df(&self, "update_time")?,
            svg_logo: self.get("svg_logo")?,
        };
        Ok(obj)
    }

    fn to_llmchat_template(self) -> AResult<LLMChatTemplate> {
        let obj = LLMChatTemplate {
            id: self.get("id")?,
            insert_time: get_df(&self, "insert_time")?,
            delete_time: get_op_df(&self, "delete_time")?,
            update_time: get_op_df(&self, "update_time")?,
            name: self.get("name")?,
            prompt: self.get("prompt")?,
            svg_logo: self.get("svg_logo")?,
        };
        Ok(obj)
    }

    fn to_llmchat_session(self) -> AResult<LLMChatSession> {
        let obj = LLMChatSession {
            id: self.get("id")?,
            insert_time: get_df(&self, "insert_time")?,
            bot_id: self.get("bot_id")?,
            template_id: self.get("template_id")?,
            title: self.get("title")?,
            namespace: self.get("namespace")?,
            delete_time: get_op_df(&self, "delete_time")?,
            update_time: get_op_df(&self, "update_time")?,
        };
        Ok(obj)
    }

    fn to_llmchat_record(self) -> AResult<LLMChatRecord> {
        let obj = LLMChatRecord {
            id: self.get("id")?,
            insert_time: get_df(&self, "insert_time")?,
            session_id: self.get("session_id")?,
            pre_record_id: self.get("pre_record_id")?,
            content: self.get("content")?,
            role: self.get("role")?,
            role_id: self.get("role_id")?,
        };
        Ok(obj)
    }

    fn to_namespace_record(self) -> AResult<NamespaceRecord> {
        let obj = NamespaceRecord {
            id: self.get("id")?,
            insert_time: get_df(&self, "insert_time")?,
            name: self.get("name")?,
            delete_time: get_op_df(&self, "delete_time")?,
            update_time: get_op_df(&self, "update_time")?,
        };
        Ok(obj)
    }

    fn to_namespace_relation(self) -> AResult<NamespaceRelation> {
        let obj = NamespaceRelation {
            id: self.get("id")?,
            insert_time: get_df(&self, "insert_time")?,
            delete_time: get_op_df(&self, "delete_time")?,
            update_time: get_op_df(&self, "update_time")?,
            sub_id: self.get("sub_id")?,
            parent_id: self.get("parent_id")?,
        };
        Ok(obj)
    }

    fn to_resource(self) -> AResult<Resource> {
        let obj = Resource {
            id: self.get("id")?,
            insert_time: get_df(&self, "insert_time")?,
            delete_time: get_op_df(&self, "delete_time")?,
            namespace: self.get("namespace")?,
            ori_filename: self.get("ori_filename")?,
            content_type: self.get("content_type")?,
        };
        Ok(obj)
    }

    fn to_kv(self) -> AResult<KV> {
        let obj = KV {
            insert_time: get_df(&self, "insert_time")?,
            key: self.get("key")?,
            value: self.get("value")?,
            update_time: get_op_df(&self, "update_time")?,
        };
        Ok(obj)
    }

    fn to_chnot_tag(self) -> AResult<ChnotTag> {
        let obj = ChnotTag {
            id: self.get("id")?,
            namespace: self.get("namespace")?,
            tag: self.get("tag")?,
            chnot_meta_id: self.get("chnot_meta_id")?,
            insert_time: get_df(&self, "insert_time")?,
            category: ChnotTagType::Common,
        };
        Ok(obj)
    }

    fn to_inline_resource(self) -> AResult<crate::model::db::resource::InlineResource> {
        let obj = InlineResource {
            id: self.get(InlineResource::field_id())?,
            name: self.get(InlineResource::field_name())?,
            content: self.get(InlineResource::field_content())?,
            content_type: self.get(InlineResource::field_content_type())?,
            delete_time: get_op_df(&self, InlineResource::field_delete_time())?,
            insert_time: get_df(&self, InlineResource::field_insert_time())?,
        };
        Ok(obj)
    }
}
