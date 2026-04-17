import { useEffect, useRef, useState } from "react";
import {
  llmchatTemplateArchive,
  llmchatTemplateCommit,
} from "@/krate/llmchat/service";
import { useLLMChatStore } from "@/krate/llmchat/store";
import { genTID } from "@/lib/id_util";
import type { ContentBlock, LLMChatSession, LLMChatTemplate } from "../po";
import { buildRecordContent } from "../po";
import type { LLMChatRecordVO } from "../vo";
import { RecordList } from "./record-list";
import { RecordAnswering } from "./record-response";
import {
  newTemplateSession,
  useAutoScroll,
  useLLMChatComStore,
  usePersistence,
  useScrollManager,
} from "./session-store";
import TemplateForm from "./template-form";
import LLMChatTemplateList from "./template-list";
import UserInput from "./user-input";

export type { LLMChatContextProps, LLMChatContextState } from "./session-store";
export { LLMChatEditorProvider, useLLMChatComStore } from "./session-store";

const SessionContainer = ({
  onPostSave,
  readonly,
}: {
  onPostSave?: (session: LLMChatSession, title?: string) => void;
  readonly: boolean;
}) => {
  const {
    bot,
    session,
    records,
    responsing,
    persistedIds,
    template,
    showTemplateForm,
    sessionOtid,
    stopGenerating,
    setRecords,
    appendRecord,
    setSession,
    setResponsing,
    setTemplate,
    setShowTemplateForm,
  } = useLLMChatComStore((store) => {
    return {
      bot: store.bot,
      session: store.session,
      records: store.records,
      responsing: store.responsing,
      persistedIds: store.persistedIds,
      template: store.template,
      showTemplateForm: store.showTemplateForm,
      sessionOtid: store.sessionOtid,
      stopGenerating: store.stopGenerating,
      setShowTemplateForm: store.setShowTemplateForm,
      setSession: store.setSession,
      setRecords: store.setRecords,
      appendRecord: store.appendRecord,
      setResponsing: store.setResponsing,
      setTemplate: store.setTemplate,
    };
  });

  const { templates } = useLLMChatStore();
  const [editingTemplate, setEditingTemplate] = useState<
    LLMChatTemplate | undefined
  >(undefined);

  useEffect(() => {
    if (!template && !session && templates.size > 0) {
      const first = templates.values().next().value;
      if (first) {
        setTemplate(first);
      }
    }
  }, [templates, template, session, setTemplate]);

  const contentRef = useRef<HTMLDivElement>(null);
  const { atBottomRef, onScroll } = useScrollManager(contentRef);

  useEffect(() => {
    if (template) {
      setTemplate(template);
      const sr = newTemplateSession(template, sessionOtid);
      setRecords(sr.records);
      setSession(sr.session);
    }
  }, [template, sessionOtid, setRecords, setSession, setTemplate]);

  usePersistence({
    session,
    records,
    readonly,
    onPostSave,
    persistedIds,
  });

  const appendUserMsg = (content: string, attachments?: ContentBlock[]) => {
    if (records && records?.length > 0 && session) {
      const recordContent = buildRecordContent(content);
      const finalParts = attachments
        ? { ...recordContent, parts: [...attachments, ...recordContent.parts] }
        : recordContent;
      const record: LLMChatRecordVO = {
        otid: genTID(),
        session_otid: session.otid,
        pre_record_otid: records.at(-1)?.otid,
        content: finalParts,
        role: "user",
        tid: genTID(),
      };
      appendRecord(record);
      setResponsing(true);
      return true;
    } else {
      return false;
    }
  };

  const { bottomDivRef, autoScrollToEnd } = useAutoScroll(atBottomRef);

  return (
    <div className="flex flex-col h-full w-full">
      <div
        className="flex flex-row flex-grow overflow-y-auto justify-center w-full"
        onScroll={onScroll}
      >
        {records && session ? (
          <div className="w-full max-w-3xl" ref={contentRef}>
            {records.length > 0 ? (
              <>
                <RecordList records={records} viewMode={readonly} />
                {session &&
                  bot &&
                  records.length > 0 &&
                  records.at(-1)?.role === "user" && (
                    <RecordAnswering
                      onScrollToEnd={() => {
                        autoScrollToEnd();
                      }}
                      bot={bot}
                    />
                  )}
                <div ref={bottomDivRef}></div>
              </>
            ) : (
              <div>None Records</div>
            )}
          </div>
        ) : (
          <div className="p-5">
            <LLMChatTemplateList
              onSelectTemplate={(template) => {
                setTemplate(template);
              }}
              onNew={(): void => {
                setEditingTemplate(undefined);
                setShowTemplateForm(true);
              }}
              onEditTemplate={(template) => {
                setEditingTemplate(template);
                setShowTemplateForm(true);
              }}
              onDeleteTemplate={async (template) => {
                if (confirm("Are you sure you want to delete this template?")) {
                  await llmchatTemplateArchive({
                    template_otid: template.otid,
                  });
                  await useLLMChatStore.getState().refreshTemplates();
                }
              }}
            />
          </div>
        )}
      </div>
      {readonly || (
        <UserInput
          disabled={responsing || records?.at(-1)?.role === "user"}
          onStopGenerating={stopGenerating}
          onAppendRecord={(content, attachments) => {
            return appendUserMsg(content, attachments);
          }}
        />
      )}
      {showTemplateForm && (
        <TemplateForm
          template={editingTemplate}
          onSubmit={async (data: LLMChatTemplate): Promise<boolean> => {
            try {
              const _ = await llmchatTemplateCommit(data);
              return true;
            } catch (_ex) {
              return false;
            } finally {
              setShowTemplateForm(false);
              setEditingTemplate(undefined);
            }
          }}
          onClose={() => {
            setShowTemplateForm(false);
            setEditingTemplate(undefined);
          }}
        />
      )}
    </div>
  );
};

export default SessionContainer;
