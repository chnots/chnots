import { TID } from "@/lib/id_util";
import {
  LLMChatBot,
  LLMChatRecord,
  LLMChatSession,
  LLMChatTemplate,
} from "../po";
import LoadingPage from "@/common/pages/loading-page";
import {
  createContext,
  createRef,
  RefObject,
  useContext,
  useState,
} from "react";
import { createStore, StoreApi, useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";

import LLMChatTemplateList from "./template-list";
import { useCallback, useEffect, useLayoutEffect, useRef } from "react";
import { RecordAnswering } from "./record-response";

import { useLLMChatStore } from "@/krate/llmchat/store";
import {
  llmchatRecordCommit,
  llmchatSessionCommit,
  llmchatSessionRecordTruncate,
  llmchatTemplateCommit,
} from "@/krate/llmchat/service";
import RecordUser from "./record-user";
import RecordAssistant, { RecordSystem } from "./record-assistant";
import UserInput from "./user-input";
import { genTID } from "@/lib/id_util";
import TemplateForm from "./template-form";
import { Dialog } from "@/common/component/ui/dialog";

export type LLMChatContextProps = {
  sessionOtid: TID;
  showTemplateForm?: boolean;
  template?: LLMChatTemplate;
  session?: LLMChatSession;
  records?: LLMChatRecord[];
  persistedIds: RefObject<Set<TID> | null>;
};

export type LLMChatContextState = {
  bot?: LLMChatBot;
  responsing: boolean;
  setSession: (session: LLMChatSession) => void;
  setRecords: (records: LLMChatRecord[]) => void;
  pushPersisted: (recordOtid: TID) => void;
  appendRecord: (record: LLMChatRecord) => void;
  updateRecord: (record: LLMChatRecord) => void;
  regenrate: (recordOtid: TID) => Promise<void>;
  setResponsing: (flag: boolean) => void;
  setTemplate: (template: LLMChatTemplate) => void;
  setBot: (bot: LLMChatBot) => void;
  setShowTemplateForm: (flag: boolean) => void;
} & LLMChatContextProps;

function createLLMChatStore(props: LLMChatContextProps) {
  return createStore<LLMChatContextState>()((set) => ({
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
        ids.current!.add(recordOtid);
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
        console.log("prev", prev.persistedIds);
        prev.persistedIds.current!.delete(record.otid);
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
      const session = this.session;
      if (session && this.records) {
        if (this.persistedIds.current!.has(recordOtid)) {
          await llmchatSessionRecordTruncate({
            session_otid: session.otid,
            remove_rid_included: recordOtid,
          });
        }
        const newRecs: LLMChatRecord[] = [];
        if (this.records) {
          for (const rec of this.records) {
            if (rec.otid === recordOtid) {
              break;
            }
            newRecs.push(rec);
          }
        }
        set((prev) => {
          return { ...prev, responsing: true };
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
): { session: LLMChatSession; records: LLMChatRecord[] } => {
  const session: LLMChatSession = {
    otid: sessionOtid || genTID(),
    template_otid: template.otid,
    title: "Untitled",
    tid: genTID(),
  };

  const record: LLMChatRecord = {
    otid: genTID(),
    session_otid: session.otid,
    content: template.prompt,
    reasoning_content: "",
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
  console.log("render SessionContainer", sessionOtid);

  const { refreshTemplates, refreshBots } = useLLMChatStore();
  useEffect(() => {
    refreshTemplates();
    refreshBots();
  }, []);

  const contentRef = useRef<HTMLDivElement>(null);
  const atBottomRef = useRef<boolean>(false);

  useEffect(() => {
    if (template) {
      setTemplate(template);
      const sr = newTemplateSession(template, sessionOtid);
      setRecords(sr.records);
      setSession(sr.session);
    }
  }, [template]);

  useEffect(() => {
    (async () => {
      if (
        session &&
        // At least one user message is inserted
        records?.some((e) => e.role === "user") &&
        !readonly
      ) {
        const pids = persistedIds.current!;
        if (!pids.has(session.otid)) {
          // As the first record is always system template.
          session.title = records[1].content.substring(0, 400);
          console.log("session,", pids);
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
            console.log("insert record", record);
            pids.add(record.otid);
          }
        }
      }
    })();
  }, [session, records, readonly]);

  const appendUserMsg = useCallback(
    (content: string) => {
      if (records && records?.length > 0 && session) {
        const record: LLMChatRecord = {
          otid: genTID(),
          session_otid: session.otid,
          pre_record_otid: records.at(-1)?.otid,
          content,
          reasoning_content: "",
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
    [records, session],
  );

  const onScroll = useCallback(() => {
    if (contentRef.current) {
      const rect = contentRef.current.getBoundingClientRect();
      // 50 is a experience value.
      const atBottom =
        Math.abs(rect.y - rect.height - 50) > contentRef.current.scrollHeight;
      atBottomRef.current = atBottom;
    }
  }, [atBottomRef]);

  const bottomDivRef = useRef<HTMLDivElement>(null);
  const scrollToEnd = useCallback((behavior: "auto" | "instant" | "smooth") => {
    bottomDivRef.current?.scrollIntoView({ behavior });
  }, []);
  const autoScrollToEnd = useCallback(() => {
    if (atBottomRef.current) {
      scrollToEnd("instant");
    }
  }, [atBottomRef, scrollToEnd]);

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
                {records.at(-1)?.role === "user" && bot && (
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
              onNew={function (): void {
                setShowTemplateForm(true);
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
          onSubmit={async function (data: LLMChatTemplate): Promise<boolean> {
            try {
              const _ = await llmchatTemplateCommit(data);
              return true;
            } catch (_ex) {
              return false;
            } finally {
              setShowTemplateForm(false);
            }
          }}
          onClose={() => {
            setShowTemplateForm(false);
          }}
        />
      )}
    </div>
  );
};

export default SessionContainer;
