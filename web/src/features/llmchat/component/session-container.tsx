import LLMChatTemplateList from "./template-list";
import React, {
  useCallback,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
} from "react";
import { v4 as uuid } from "uuid";
import { useKSpaceStore } from "@/store/kspace";
import { RecordAnswering } from "./record-response";
import { LLMChatRecord, LLMChatTemplate } from "@/store/llmchat/db";
import {
  LLMChatContainerSession,
  LLMChatSessionDetailRsp,
} from "@/store/llmchat/dto";
import { useLLMChatStore } from "@/store/llmchat/store";
import {
  llmchatRecordInsert,
  llmchatSessionRecords,
  llmchatSessionTruncate,
} from "@/store/llmchat/service";
import RecordUser from "./record-user";
import RecordAssistant from "./record-assistant";
import LLMChatSessionInput from "./session-input";

const SessionContainer = ({
  sessionIdOrUUID,
  onNewButton,
  afterInit,
}: {
  sessionIdOrUUID: string;
  onNewButton: () => void;
  afterInit?: (id: string) => void;
}) => {
  const { currentBot, unshiftSession } = useLLMChatStore();
  const { currentKSpace } = useKSpaceStore();

  const [containerSession, setContainerSession] =
    useState<LLMChatContainerSession>();
  const [triggerAnswer, setTriggerAnswer] = useState<boolean>(false);
  const [responsing, setResponsing] = useState<boolean>(false);
  const [responseId, setResponseId] = useState<string>(uuid());

  const contentRef = useRef<HTMLDivElement>(null);
  const atBottomRef = useRef<boolean>(false);

  // Used to load from database.
  useEffect(() => {
    if (sessionIdOrUUID) {
      llmchatSessionRecords(sessionIdOrUUID).then(
        (rsp: LLMChatSessionDetailRsp) => {
          if (rsp.session) {
            const pids = new Set(rsp.records.map((e) => e.id));
            pids.add(rsp.session.id);
            setContainerSession({
              session: rsp.session,
              records: rsp.records,
              persistedIds: pids,
            });
          }
        }
      );
    }
  }, [responseId]);

  const newTemplateSession = useCallback(
    async (template: LLMChatTemplate) => {
      const session = {
        id: uuid(),
        bot_id: currentBot ? currentBot.id : "1",
        template_id: template.id,
        title: "Untitled",
        kspace: currentKSpace.name,
        insert_time: new Date(),
      };

      const record: LLMChatRecord = {
        id: uuid(),
        session_id: session.id,
        content: template.prompt,
        reasoning_content: "",
        role_id: template.id,
        role: "system",
        insert_time: new Date(),
      };
      setContainerSession({
        records: [record],
        session: session,
        persistedIds: new Set(),
      });
    },
    [setContainerSession]
  );

  useEffect(() => {
    const save = async () => {
      if (
        containerSession &&
        // At least one user message is inserted
        containerSession.records.some((e) => e.role === "user")
      ) {
        const session = containerSession.session;
        const records = containerSession.records;
        const pids = containerSession.persistedIds;
        let added = false;
        if (!pids.has(session.id)) {
          // As the first record is always system template.
          session.title = records[1].content.substring(0, 400);
          await unshiftSession(session);
          pids.add(session.id);
          if (afterInit) {
            afterInit(session.id);
          }
          added = true;
        }
        for (const record of records) {
          if (!pids.has(record.id)) {
            await llmchatRecordInsert(record);
            console.log("insert record", record);
            pids.add(record.id);
            added = true;
          }
        }
        if (added) {
          setContainerSession((prev) => {
            if (prev) {
              return {
                ...prev,
                persistedIds: pids,
              };
            } else {
              return undefined;
            }
          });
        }
      }
    };
    save();
  }, [containerSession]);

  const appendRecord = useCallback(
    async (record: LLMChatRecord) => {
      setContainerSession((prev) => {
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
    },
    [setContainerSession]
  );

  const truncateSession = useCallback(
    async (recordId: string) => {
      if (containerSession) {
        if (containerSession.persistedIds.has(recordId)) {
          await llmchatSessionTruncate({
            session_id: containerSession.session.id,
            remove_rid_included: recordId,
          });
        }
        setResponseId(uuid());
        setTriggerAnswer(true);
      }
    },
    [containerSession]
  );

  const appendUserMsg = useCallback(
    (content: string) => {
      if (containerSession && containerSession.records.length > 0) {
        const record: LLMChatRecord = {
          id: uuid(),
          session_id: containerSession.session.id,
          pre_record_id: containerSession.records.at(-1)?.id,
          content,
          reasoning_content: "",
          role: "user",
          insert_time: new Date(),
        };
        appendRecord(record);
        setTriggerAnswer(true);
        return true;
      } else {
        return false;
      }
    },
    [containerSession, setTriggerAnswer]
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
  const scrollToEnd = useCallback(() => {
    console.log("scroll to end", bottomDivRef.current);

    bottomDivRef.current?.scrollIntoView({ behavior: "smooth" });
  }, []);

  useLayoutEffect(() => {
    scrollToEnd();
  });

  return (
    <div className="bg-active flex flex-col h-full max-h-full overflow-hidden rounded-md shadow">
      <div
        className="flex flex-row h-full overflow-y-auto justify-center w-full"
        onScroll={onScroll}
      >
        {currentBot ? (
          containerSession ? (
            <div className="w-full max-w-3xl" ref={contentRef}>
              {containerSession.records.length > 0 ? (
                <>
                  {containerSession.records
                    .toSorted((a, b) => {
                      return a.insert_time > b.insert_time ? 1 : -1;
                    })
                    .map((record) => {
                      return record.role === "user" ? (
                        <RecordUser record={record} key={record.id} />
                      ) : (
                        <RecordAssistant
                          {...record}
                          key={record.id}
                          onRegenerate={
                            record.role === "assistant"
                              ? async () => {
                                  truncateSession(record.id);
                                }
                              : undefined
                          }
                        />
                      );
                    })}
                  {containerSession.records.at(-1)?.role === "user" && (
                    <RecordAnswering
                      key={responseId}
                      containerSession={containerSession}
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
                        scrollToEnd();
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
            <div className={"flex flex-col h-full justify-center"}>
              <LLMChatTemplateList
                onClickTemplate={(template) => {
                  newTemplateSession(template);
                }}
              />
            </div>
          )
        ) : (
          <div>Please add a bot</div>
        )}
      </div>
      <LLMChatSessionInput
        disabled={
          responsing ||
          !containerSession ||
          containerSession.records?.at(-1)?.role === "user"
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
