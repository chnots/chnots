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
} from "@/krate/llmchat/service";
import { genTID, type TID } from "@/lib/id_util";
import type { LLMChatBot, LLMChatSession, LLMChatTemplate } from "../po";
import type { LLMChatRecordVO } from "../vo";

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
  stopGenerating: (() => void) | null;
  setSession: (session: LLMChatSession) => void;
  setRecords: (records: LLMChatRecordVO[]) => void;
  pushPersisted: (recordOtid: TID) => void;
  appendRecord: (record: LLMChatRecordVO) => void;
  updateRecord: (record: LLMChatRecordVO) => void;
  regenrate: (recordOtid: TID) => Promise<void>;
  setResponsing: (flag: boolean) => void;
  setStopGenerating: (fn: (() => void) | null) => void;
  setTemplate: (template: LLMChatTemplate) => void;
  setBot: (bot: LLMChatBot) => void;
  setShowTemplateForm: (flag: boolean) => void;
} & LLMChatContextProps;

function createLLMChatStore(props: LLMChatContextProps) {
  return createStore<LLMChatContextState>()((set, get) => ({
    ...props,
    responsing: false,
    stopGenerating: null,
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
    setStopGenerating(fn) {
      set((prev) => {
        return { ...prev, stopGenerating: fn };
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
        const sortedRecords = self.records.toSorted((a, b) => {
          return a.otid > b.otid ? 1 : -1;
        });
        const recordIndex = sortedRecords.findIndex((rec) => {
          return rec.otid === recordOtid;
        });
        if (recordIndex < 0) {
          return;
        }

        const newRecs = sortedRecords.slice(0, recordIndex);
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

  useEffect(() => {
    if (props.records !== undefined) {
      store.getState().setRecords(props.records);
    }
  }, [props.records, store]);

  return store ? (
    <LLMChatEditorContext.Provider value={store}>
      {children}
    </LLMChatEditorContext.Provider>
  ) : (
    <LoadingPage />
  );
}

export const newTemplateSession = (
  template: LLMChatTemplate,
  sessionOtid?: TID,
): { session: LLMChatSession; records: LLMChatRecordVO[] } => {
  const session: LLMChatSession = {
    otid: sessionOtid || genTID(),
    template_otid: template.otid,
    tid: genTID(),
  };

  const record: LLMChatRecordVO = {
    otid: genTID(),
    session_otid: session.otid,
    content: { usage: {}, parts: [{ type: "content", data: template.prompt }] },
    role_id: template.otid,
    role: "system",
    tid: genTID(),
  };

  return {
    session: session,
    records: [record],
  };
};

export const buildSessionTitle = (
  records?: LLMChatRecordVO[],
): string | undefined => {
  if (!records || records.length === 0) {
    return undefined;
  }

  const firstUserRecord = records
    .toSorted((a, b) => {
      return a.otid > b.otid ? 1 : -1;
    })
    .find((record) => {
      return record.role === "user";
    });

  if (!firstUserRecord) {
    return undefined;
  }

  const body =
    firstUserRecord.content.parts.find((b) => b.type === "content")?.data ?? "";
  const normalized = body.replaceAll(/\s+/g, " ").trim();
  if (!normalized) {
    return undefined;
  }

  return normalized.slice(0, 80);
};

export function useScrollManager(contentRef: RefObject<HTMLDivElement | null>) {
  const atBottomRef = useRef<boolean>(false);

  const onScroll = useCallback(() => {
    const container = contentRef.current?.parentElement;
    if (container) {
      const { scrollTop, scrollHeight, clientHeight } = container;
      atBottomRef.current = scrollHeight - scrollTop - clientHeight < 50;
    }
  }, [contentRef]);

  return { atBottomRef, onScroll };
}

export function useAutoScroll(atBottomRef: RefObject<boolean | null>) {
  const bottomDivRef = useRef<HTMLDivElement>(null);

  const scrollToEnd = useCallback((behavior: "auto" | "instant" | "smooth") => {
    bottomDivRef.current?.scrollIntoView({ behavior });
  }, []);

  const autoScrollToEnd = useCallback(() => {
    if (atBottomRef.current) {
      scrollToEnd("instant");
    }
  }, [scrollToEnd, atBottomRef]);

  return { bottomDivRef, scrollToEnd, autoScrollToEnd };
}

export function usePersistence({
  session,
  records,
  readonly,
  onPostSave,
  persistedIds,
}: {
  session?: LLMChatSession;
  records?: LLMChatRecordVO[];
  readonly: boolean;
  onPostSave?: (session: LLMChatSession, title?: string) => void;
  persistedIds: RefObject<Set<TID> | null>;
}) {
  useEffect(() => {
    (async () => {
      if (
        session &&
        records?.some((e) => e.role === "user") &&
        !readonly &&
        persistedIds.current
      ) {
        const pids = persistedIds.current;
        if (!pids.has(session.otid)) {
          await llmchatSessionCommit({
            session: session,
          });
          pids.add(session.otid);
          if (onPostSave) {
            onPostSave(session, buildSessionTitle(records));
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
}
