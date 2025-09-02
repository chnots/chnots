import { TID } from "@/lib/id_util";
import {
  LLMChatBot,
  LLMChatRecord,
  LLMChatSession,
  LLMChatTemplate,
} from "../po";
import LoadingPage from "@/common/pages/loading-page";
import { createContext, useContext, useState } from "react";
import { createStore, StoreApi, useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";

import LLMChatTemplateList from "./template-list";
import { useCallback, useEffect, useLayoutEffect, useRef } from "react";
import { RecordAnswering } from "./record-response";

import { useLLMChatStore } from "@/krate/llmchat/store";
import {
  llmchatRecordInsert,
  llmchatSessionRecords,
  llmchatSessionTruncate,
  llmchatTemplateAdd,
} from "@/krate/llmchat/service";
import RecordUser from "./record-user";
import RecordAssistant from "./record-assistant";
import LLMChatSessionInput from "./session-input";
import { Dialog, DialogContent } from "@/common/component/ui/dialog";
import TemplateForm from "./template-form";
import { genTID } from "@/lib/id_util";
import { DialogTitle } from "@radix-ui/react-dialog";

export type LLMChatContextProps = {
  sessionOtid: TID;
  viewMode: boolean;
};

export type LLMChatContextState = {
  bot?: LLMChatBot;
  template?: LLMChatTemplate;
  session?: LLMChatSession;
  records?: LLMChatRecord[];
  persistedIds: Set<TID>;
  responsing: boolean;
  setSession: (session: LLMChatSession) => void;
  setRecords: (records: LLMChatRecord[]) => void;
  pushPersisted: (recordOtid: TID) => void;
  appendRecord: (record: LLMChatRecord) => void;
  regenrate: (recordOtid: TID) => Promise<void>;
  setResponsing: (flag: boolean) => void;
} & LLMChatContextProps;

function createLLMChatStore(props: LLMChatContextProps) {
  return createStore<LLMChatContextState>()((set) => ({
    ...props,
    persistedIds: new Set(),
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
    pushPersisted(recordOtid) {
      set((prev) => {
        const ids = prev.persistedIds;
        ids.add(recordOtid);
        return { ...prev, persistedIds: ids };
      });
    },

    appendRecord(record) {
      set((prev) => {
        return { ...prev, records: [...(prev.records ?? []), record] };
      });
    },
    async regenrate(recordOtid) {
      const session = this.session;
      if (session && this.records) {
        if (this.persistedIds.has(recordOtid)) {
          await llmchatSessionTruncate({
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
  }));
}

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
  kindId,
  onAfterSave,
}: {
  kindId?: string;
  onAfterSave?: (session: LLMChatSession) => void;
}) => {
  const { currentBot, unshiftSession } = useLLMChatStore();
  const {
    session,
    records,
    responsing,
    setRecords,
    appendRecord,
    setSession,
    setResponsing,
  } = useLLMChatComStore((store) => {
    return {
      session: store.session,
      records: store.records,
      setSession: store.setSession,
      setRecords: store.setRecords,
      appendRecord: store.appendRecord,
      setResponsing: store.setResponsing,
      responsing: store.responsing,
    };
  });
  const { refreshTemplates, refreshBots } = useLLMChatStore();
  useEffect(() => {
    refreshTemplates();
    refreshBots();
  }, []);

  const persistedIds = useRef<Set<TID>>(new Set());

  const contentRef = useRef<HTMLDivElement>(null);
  const atBottomRef = useRef<boolean>(false);
  const [editTemplate, setEditTemplate] = useState<LLMChatTemplate>();

  // Used to load from database.
  useEffect(() => {
    (async () => {
      if (kindId) {
        const rsp = await llmchatSessionRecords(parseInt(kindId, 10));

        if (rsp.session) {
          const pids = persistedIds.current;
          rsp.records.forEach((r) => {
            pids.add(r.otid);
          });
          pids.add(rsp.session.otid);
        }
      }
    })();
  }, [kindId]);

  const newTemplateSession = useCallback(async (template: LLMChatTemplate) => {
    const session: LLMChatSession = {
      otid: genTID(),
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
    setSession(session);
    setRecords([record]);
  }, []);

  useEffect(() => {
    const save = async () => {
      if (
        session &&
        // At least one user message is inserted
        records?.some((e) => e.role === "user")
      ) {
        const pids = persistedIds.current;
        console.log("pids: ", pids, session.otid);
        if (!pids.has(session.otid)) {
          // As the first record is always system template.
          session.title = records[1].content.substring(0, 400);
          await unshiftSession(session);
          pids.add(session.otid);
          if (onAfterSave) {
            onAfterSave(session);
          }
        }
        for (const record of records) {
          if (!pids.has(record.otid)) {
            await llmchatRecordInsert(record);
            console.log("insert record", record);
            pids.add(record.otid);
          }
        }
      }
    };
    save();
  }, [session, records]);

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

  useLayoutEffect(() => {
    scrollToEnd("smooth");
  });

  return (
    <div className="flex flex-col h-full max-h-full overflow-hidden rounded-md shadow">
      <div
        className="flex flex-row h-full overflow-y-auto justify-center w-full"
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
                      <RecordUser record={record} key={record.otid} />
                    ) : (
                      <RecordAssistant
                        timestamp={new Date(record.otid / 1e3).toISOString()}
                        {...record}
                        key={record.otid}
                      />
                    );
                  })}
                {records.at(-1)?.role === "user" && currentBot && (
                  <RecordAnswering
                    onScrollToEnd={() => {
                      autoScrollToEnd();
                    }}
                    bot={currentBot}
                  />
                )}
                <div ref={bottomDivRef}></div>
              </>
            ) : (
              <div>None Records</div>
            )}
          </div>
        ) : (
          <Dialog>
            <LLMChatTemplateList
              onClickTemplate={(template) => {
                newTemplateSession(template);
              }}
              onChangeEditTemplate={(template: LLMChatTemplate) => {
                setEditTemplate(template);
              }}
            />
            <DialogContent aria-describedby={undefined}>
              <DialogTitle>Edit Template</DialogTitle>
              <TemplateForm
                onSubmit={async (data: LLMChatTemplate): Promise<boolean> => {
                  await llmchatTemplateAdd(data);
                  return true;
                }}
                template={editTemplate}
              />
            </DialogContent>
          </Dialog>
        )}
      </div>
      <LLMChatSessionInput
        disabled={responsing || records?.at(-1)?.role === "user"}
        onAppendRecord={(content) => {
          return appendUserMsg(content);
        }}
      />
    </div>
  );
};

export default SessionContainer;
