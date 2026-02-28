import {
  createContext,
  type RefObject,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";
import { createStore, type StoreApi, useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";
import LoadingPage from "@/common/pages/loading-page";
import {
  llmchatRecordCommit,
  llmchatSessionCommit,
  llmchatSessionRecordTruncate,
  llmchatTemplateArchive,
  llmchatTemplateCommit,
} from "@/krate/llmchat/service";
import { useLLMChatStore } from "@/krate/llmchat/store";
import { genTID, type TID } from "@/lib/id_util";
import type { LLMChatBot, LLMChatSession, LLMChatTemplate } from "../po";
import type { LLMChatRecordVO } from "../vo";
import RecordAssistant, { RecordSystem } from "./record-assistant";
import { RecordAnswering } from "./record-response";
import RecordUser from "./record-user";
import TemplateForm from "./template-form";
import LLMChatTemplateList from "./template-list";
import UserInput from "./user-input";

export type LLMChatContextProps = {
  sessionOtid: TID;
  showTemplateForm?: boolean;
  template?: LLMChatTemplate;
  session?: LLMChatSession;
  records?: LLMChatRecordVO[];
  persistedIds: RefObject<Set<TID> | null>;
};

export type LLMChatContextState = {
  bot?: LLMChatBot;
  responsing: boolean;
  setSession: (session: LLMChatSession) => void;
  setRecords: (records: LLMChatRecordVO[]) => void;
  pushPersisted: (recordOtid: TID) => void;
  appendRecord: (record: LLMChatRecordVO) => void;
  updateRecord: (record: LLMChatRecordVO) => void;
  regenrate: (recordOtid: TID) => Promise<void>;
  setResponsing: (flag: boolean) => void;
  setTemplate: (template: LLMChatTemplate) => void;
  setBot: (bot: LLMChatBot) => void;
  setShowTemplateForm: (flag: boolean) => void;
} & LLMChatContextProps;

function createLLMChatStore(props: LLMChatContextProps) {
  return createStore<LLMChatContextState>()((set, get) => ({
    ...props,
    responsing: false,
    setSession: (session: LLMChatSession) => {
      set((prev) => {
        return { ...prev, session: session };
      });
    },
    setRecords(records) {
      set((prev) => {
        return { ...prev, records: records };
      });
    },
    setResponsing(flag) {
      set((prev) => {
        return { ...prev, responsing: flag };
      });
    },
    setBot(bot) {
      set((prev) => {
        return { ...prev, bot: bot };
      });
    },
    setShowTemplateForm(flag: boolean) {
      set((prev) => {
        return { ...prev, showTemplateForm: flag };
      });
    },
    pushPersisted(recordOtid) {
      set((prev) => {
        const ids = prev.persistedIds;
        ids.current?.add(recordOtid);
        return { ...prev, persistedIds: ids };
      });
    },
    appendRecord(record) {
      set((prev) => {
        return { ...prev, records: [...(prev.records ?? []), record] };
      });
    },
    updateRecord(record) {
      set((prev) => {
        prev.persistedIds.current?.delete(record.otid);
        return {
          ...prev,
          records: prev.records?.map((e) => {
            if (e.otid === record.otid) {
              return record;
            } else {
              return e;
            }
          }),
        };
      });
    },
    async regenrate(recordOtid) {
      const self = get();
      const session = self.session;
      if (session && self.records) {
        if (self.persistedIds.current?.has(recordOtid)) {
          await llmchatSessionRecordTruncate({
            session_otid: session.otid,
            remove_otid_included: recordOtid,
          });
        }
        const newRecs: LLMChatRecordVO[] = [];
        if (self.records) {
          for (const rec of self.records) {
            if (rec.otid === recordOtid) {
              break;
            }
            newRecs.push(rec);
          }
        }
        console.log("regenerate: ", newRecs);
        set((prev) => {
          return { ...prev, records: newRecs, responsing: true };
        });
      }
    },
    setTemplate(template) {
      set((prev) => {
        return { ...prev, template: template };
      });
    },
  }));
}

export const newTemplateSession = (
  template: LLMChatTemplate,
  sessionOtid?: TID,
): { session: LLMChatSession; records: LLMChatRecordVO[] } => {
  const session: LLMChatSession = {
    otid: sessionOtid || genTID(),
    template_otid: template.otid,
    title: "Untitled",
    tid: genTID(),
  };

  const record: LLMChatRecordVO = {
    otid: genTID(),
    session_otid: session.otid,
    body: template.prompt,
    thinking: "",
    role_id: template.otid,
    role: "system",
    tid: genTID(),
  };

  return {
    session: session,
    records: [record],
  };
};

const LLMChatEditorContext =
  createContext<StoreApi<LLMChatContextState> | null>(null);

export function useLLMChatComStore<T>(
  selector: (state: LLMChatContextState) => T,
) {
  const store = useContext(LLMChatEditorContext);

  return useStore(
    store!,
    useShallow((store) => {
      return selector(store);
    }),
  );
}

export function LLMChatEditorProvider({
  props,
  children,
}: {
  props: LLMChatContextProps;
  children: React.ReactNode;
}) {
  const [store] = useState<StoreApi<LLMChatContextState>>(
    createLLMChatStore(props),
  );

  return store ? (
    <LLMChatEditorContext.Provider value={store}>
      {children}
    </LLMChatEditorContext.Provider>
  ) : (
    <LoadingPage />
  );
}

const SessionContainer = ({
  onPostSave,
  readonly,
}: {
  onPostSave?: (session: LLMChatSession) => void;
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
      setShowTemplateForm: store.setShowTemplateForm,
      setSession: store.setSession,
      setRecords: store.setRecords,
      appendRecord: store.appendRecord,
      setResponsing: store.setResponsing,
      setTemplate: store.setTemplate,
    };
  });

  const { refreshTemplates, refreshBots } = useLLMChatStore();
  const [editingTemplate, setEditingTemplate] = useState<
    LLMChatTemplate | undefined
  >(undefined);

  useEffect(() => {
    refreshTemplates();
    refreshBots();
  }, [refreshBots, refreshTemplates]);

  const contentRef = useRef<HTMLDivElement>(null);
  const atBottomRef = useRef<boolean>(false);

  useEffect(() => {
    if (template) {
      setTemplate(template);
      const sr = newTemplateSession(template, sessionOtid);
      setRecords(sr.records);
      setSession(sr.session);
    }
  }, [template, sessionOtid, setRecords, setSession, setTemplate]);

  useEffect(() => {
    (async () => {
      if (
        session &&
        // At least one user message is inserted
        records?.some((e) => e.role === "user") &&
        !readonly &&
        persistedIds.current
      ) {
        const pids = persistedIds.current;
        if (!pids.has(session.otid)) {
          // As the first record is always system template.
          session.title = records[1].body.substring(0, 400);
          await llmchatSessionCommit({
            session: session,
          });
          pids.add(session.otid);
          if (onPostSave) {
            onPostSave(session);
          }
        }
        for (const record of records) {
          if (!pids.has(record.otid)) {
            await llmchatRecordCommit(record);
            pids.add(record.otid);
          }
        }
      }
    })();
  }, [session, records, readonly, onPostSave, persistedIds.current]);

  const appendUserMsg = useCallback(
    (content: string) => {
      if (records && records?.length > 0 && session) {
        const record: LLMChatRecordVO = {
          otid: genTID(),
          session_otid: session.otid,
          pre_record_otid: records.at(-1)?.otid,
          body: content,
          thinking: "",
          role: "user",
          tid: genTID(),
        };
        appendRecord(record);
        setResponsing(true);
        return true;
      } else {
        return false;
      }
    },
    [records, session, appendRecord, setResponsing],
  );

  const onScroll = useCallback(() => {
    if (contentRef.current) {
      const rect = contentRef.current.getBoundingClientRect();
      // 50 is a experience value.
      const atBottom =
        Math.abs(rect.y - rect.height - 50) > contentRef.current.scrollHeight;
      atBottomRef.current = atBottom;
    }
  }, []);

  const bottomDivRef = useRef<HTMLDivElement>(null);
  const scrollToEnd = useCallback((behavior: "auto" | "instant" | "smooth") => {
    bottomDivRef.current?.scrollIntoView({ behavior });
  }, []);
  const autoScrollToEnd = useCallback(() => {
    if (atBottomRef.current) {
      scrollToEnd("instant");
    }
  }, [scrollToEnd]);

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
                {records
                  .toSorted((a, b) => {
                    return a.otid > b.otid ? 1 : -1;
                  })
                  .map((record) => {
                    return record.role === "user" ? (
                      <RecordUser
                        viewMode={readonly}
                        record={record}
                        key={record.otid}
                      />
                    ) : record.role === "system" ? (
                      <RecordSystem
                        viewMode={readonly}
                        timestamp={new Date(record.otid / 1e3).toISOString()}
                        {...record}
                        key={record.otid}
                      />
                    ) : (
                      <RecordAssistant
                        viewMode={readonly}
                        timestamp={new Date(record.otid / 1e3).toISOString()}
                        {...record}
                        key={record.otid}
                      />
                    );
                  })}
                {session &&
                  bot &&
                  records &&
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
                  await refreshTemplates();
                }
              }}
            />
          </div>
        )}
      </div>
      {readonly || (
        <UserInput
          disabled={responsing || records?.at(-1)?.role === "user"}
          onAppendRecord={(content) => {
            return appendUserMsg(content);
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
