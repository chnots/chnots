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
            pids.add(r.otid);
          });
          pids.add(rsp.session.otid);
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

  const truncateAndRegen = useCallback(
    async (recordId: TID) => {
      if (sessionAndRecs) {
        if (persistedIds.current.has(recordId)) {
          await llmchatSessionTruncate({
            session_otid: sessionAndRecs.session.otid,
            remove_rid_included: recordId,
          });
        }
        setSessionAndRecs((prev) => {
          const newRecs: LLMChatRecord[] = [];
          if (prev?.records) {
            for (const rec of prev.records) {
              if (rec.otid === recordId) {
                break;
              }
              newRecs.push(rec);
            }
          }
          return { session: prev!.session, records: newRecs };
        });
        setTriggerAnswer(true);
        setResponseId(genTID());
      }
    },
    [sessionAndRecs],
  );

  const appendUserMsg = useCallback(
    (content: string) => {
      if (sessionAndRecs && sessionAndRecs.records.length > 0) {
        const record: LLMChatRecord = {
          otid: genTID(),
          session_otid: sessionAndRecs.session.otid,
          pre_record_otid: sessionAndRecs.records.at(-1)?.otid,
          content,
          reasoning_content: "",
          role: "user",
          tid: genTID(),
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
                        onRegenerate={
                          record.role === "assistant"
                            ? async () => {
                                truncateAndRegen(record.otid);
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
                        truncateAndRegen(responseId);
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
