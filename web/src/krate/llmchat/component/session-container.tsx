import LLMChatTemplateList from "./template-list";
import {
  useCallback,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
} from "react";
import { RecordAnswering } from "./record-response";
import {
  LLMChatRecord,
  LLMChatSession,
  LLMChatTemplate,
} from "@/krate/llmchat/po";
import {
  LLMChatSessionDetail,
  LLMChatSessionDetailRsp,
} from "@/krate/llmchat/dto";
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
import {
  Dialog,
  DialogClose,
  DialogContent,
} from "@/common/component/ui/dialog";
import TemplateForm from "./template-form";
import { genTID, TID } from "@/lib/id_util";
import { DialogTitle } from "@radix-ui/react-dialog";

const SessionContainer = ({
  kindId,
  onNewButton,
  onAfterSave,
}: {
  kindId?: string;
  onNewButton?: () => void;
  onAfterSave?: (session: LLMChatSession) => void;
}) => {
  console.log("sessions: ", kindId);
  const { currentBot, unshiftSession } = useLLMChatStore();
  const { refreshTemplates, refreshBots } = useLLMChatStore();
  useEffect(() => {
    refreshTemplates();
    refreshBots();
  }, []);

  const [sessionAndRecs, setSessionAndRecs] = useState<LLMChatSessionDetail>();
  const persistedIds = useRef<Set<TID>>(new Set());

  const [triggerAnswer, setTriggerAnswer] = useState<boolean>(false);
  const [responsing, setResponsing] = useState<boolean>(false);
  const [responseId, setResponseId] = useState<TID>(genTID());

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
            pids.add(r.tid);
          });
          pids.add(rsp.session.tid);
          setSessionAndRecs({
            session: rsp.session,
            records: rsp.records,
          });
        }
      }
    })();
  }, [kindId]);

  const newTemplateSession = useCallback(
    async (template: LLMChatTemplate) => {
      const session: LLMChatSession = {
        tid: genTID(),
        template_id: template.tid,
        title: "Untitled",
      };

      const record: LLMChatRecord = {
        tid: genTID(),
        session_id: session.tid,
        content: template.prompt,
        reasoning_content: "",
        role_id: template.tid,
        role: "system",
      };
      setSessionAndRecs({
        records: [record],
        session: session,
      });
    },
    [setSessionAndRecs],
  );

  useEffect(() => {
    const save = async () => {
      if (
        sessionAndRecs &&
        // At least one user message is inserted
        sessionAndRecs.records.some((e) => e.role === "user")
      ) {
        const session = sessionAndRecs.session;
        const records = sessionAndRecs.records;

        const pids = persistedIds.current;
        console.log("pids: ", pids, session.tid);
        if (!pids.has(session.tid)) {
          // As the first record is always system template.
          session.title = records[1].content.substring(0, 400);
          await unshiftSession(session);
          pids.add(session.tid);
          if (onAfterSave) {
            onAfterSave(session);
          }
        }
        for (const record of records) {
          if (!pids.has(record.tid)) {
            await llmchatRecordInsert(record);
            console.log("insert record", record);
            pids.add(record.tid);
          }
        }
      }
    };
    save();
  }, [sessionAndRecs]);

  const appendRecord = useCallback(async (record: LLMChatRecord) => {
    setSessionAndRecs((prev) => {
      if (prev) {
        return {
          ...prev,
          records: [...prev.records, record],
        };
      } else {
        return undefined;
      }
    });
    return true;
  }, []);

  const truncateSession = useCallback(
    async (recordId: TID) => {
      if (sessionAndRecs) {
        if (persistedIds.current.has(recordId)) {
          await llmchatSessionTruncate({
            session_id: sessionAndRecs.session.tid,
            remove_rid_included: recordId,
          });
        }
        setResponseId(genTID());
        setTriggerAnswer(true);
      }
    },
    [sessionAndRecs],
  );

  const appendUserMsg = useCallback(
    (content: string) => {
      if (sessionAndRecs && sessionAndRecs.records.length > 0) {
        const record: LLMChatRecord = {
          tid: genTID(),
          session_id: sessionAndRecs.session.tid,
          pre_record_id: sessionAndRecs.records.at(-1)?.tid,
          content,
          reasoning_content: "",
          role: "user",
        };
        appendRecord(record);
        setTriggerAnswer(true);
        return true;
      } else {
        return false;
      }
    },
    [sessionAndRecs, setTriggerAnswer],
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
        {sessionAndRecs ? (
          <div className="w-full max-w-3xl" ref={contentRef}>
            {sessionAndRecs.records.length > 0 ? (
              <>
                {sessionAndRecs.records
                  .toSorted((a, b) => {
                    return a.tid > b.tid ? 1 : -1;
                  })
                  .map((record) => {
                    return record.role === "user" ? (
                      <RecordUser record={record} key={record.tid} />
                    ) : (
                      <RecordAssistant
                        {...record}
                        key={record.tid}
                        onRegenerate={
                          record.role === "assistant"
                            ? async () => {
                                truncateSession(record.tid);
                              }
                            : undefined
                        }
                      />
                    );
                  })}
                {sessionAndRecs.records.at(-1)?.role === "user" &&
                  currentBot && (
                    <RecordAnswering
                      key={responseId}
                      containerSession={sessionAndRecs}
                      triggerAnswer={triggerAnswer}
                      bot={currentBot}
                      onRegenerate={() => {
                        truncateSession(responseId);
                      }}
                      onSetResponsing={(flag) => {
                        setResponsing(flag);
                      }}
                      onEnd={(r) => {
                        appendRecord(r);
                      }}
                      onScrollToEnd={() => {
                        autoScrollToEnd();
                      }}
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
        disabled={
          responsing ||
          !sessionAndRecs ||
          sessionAndRecs.records?.at(-1)?.role === "user"
        }
        onAppendRecord={(content) => {
          return appendUserMsg(content);
        }}
        onNewButton={onNewButton}
      />
    </div>
  );
};

export default SessionContainer;
