import LLMChatTemplateList from "./template-list";
import { RefObject, useCallback, useEffect, useRef, useState } from "react";
import { v4 as uuid } from "uuid";
import { useNamespaceStore } from "@/store/namespace";
import LLMChatSessionInput from "./session-input";
import { Record } from "./record";
import { ResponseRecord } from "./response-record";
import LLMChatBotSelect from "./bot-select";
import {
  LLMChatRecord,
  LLMChatTemplate,
  LLMChatSession,
} from "@/store/llmchat/db";
import {
  LLMChatSessionDetail,
  LLMChatSessionDetailRsp,
} from "@/store/llmchat/dto";
import { useLLMChatStore } from "@/store/llmchat/store";
import {
  llmchatRecordInsert,
  llmchatSessionRecords,
} from "@/store/llmchat/service";

const LLMChatSessionBody = ({
  newSessionFlag,
}: {
  newSessionFlag?: number;
}) => {
  const { currentSession, currentBot, setCurrentSession, unshiftSession } =
    useLLMChatStore();
  const { currentNamespace } = useNamespaceStore();

  const [fleetDetail, setFleetDetail] = useState<LLMChatSessionDetail>();
  const [answering, setAnswering] = useState<boolean>(false);
  const [triggerAnswer, setTriggerAnswer] = useState<boolean>(false);
  const [refreshTrigger, setRefreshTrigger] = useState<number>(0);

  const refresh = useCallback(() => {
    if (currentSession)
      llmchatSessionRecords(currentSession.id).then(
        (rsp: LLMChatSessionDetailRsp) => {
          setFleetDetail({
            session: currentSession,
            records: rsp.records,
            persisted: true,
          });
        }
      );
  }, [currentSession, fleetDetail, setFleetDetail]);

  useEffect(() => {
    if (currentSession && currentSession.id !== fleetDetail?.session?.id) {
      refresh();
    }
  }, [currentSession, fleetDetail]);

  useEffect(() => {
    refresh();
  }, [refreshTrigger]);

  useEffect(() => {
    setFleetDetail(undefined);
  }, [currentSession, newSessionFlag]);

  const trySaveSession = async (
    detail: LLMChatSessionDetail,
    title: string
  ) => {
    if (!detail.persisted) {
      detail.session.title = title.substring(0, 400);
      await unshiftSession(detail.session);
      for (const record of detail.records) {
        await llmchatRecordInsert(record);
      }
      setFleetDetail({ ...detail, persisted: true });
      setCurrentSession(detail.session);
    }
  };

  const appendRecord = async (record: LLMChatRecord) => {
    await llmchatRecordInsert(record);
    setFleetDetail((prev) => {
      if (prev) {
        return {
          ...prev,
          records: [...prev.records, record],
          persisted: true,
        };
      } else {
        return undefined;
      }
    });
    return true;
  };

  const initFleetSession = async (template: LLMChatTemplate) => {
    const session: LLMChatSession = {
      id: uuid(),
      bot_id: currentBot ? currentBot.id : "1",
      template_id: template.id,
      title: "Untitled",
      namespace: currentNamespace.name,
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

    setFleetDetail({ records: [record], persisted: false, session: session });
  };

  const saveSessionAndRecord = async (
    record: LLMChatRecord,
    session?: LLMChatSessionDetail
  ) => {
    if (session) {
      await trySaveSession(session, record.content);
      await appendRecord(record);
      setTriggerAnswer(true);
      return true;
    } else {
      return false;
    }
  };

  const contentRef = useRef<HTMLDivElement>(null);
  const [atBottom, setAtBottom] = useState(true);
  const handleScroll = useCallback(() => {
    if (contentRef.current) {
      const rect = contentRef.current.getBoundingClientRect();
      // 50 is a experience value.
      const atBottom =
        Math.abs(rect.y - rect.height - 50) > contentRef.current.scrollHeight;
      setAtBottom(atBottom);
    }
  }, [setAtBottom]);
  useEffect(() => {
    if (contentRef.current) {
      console.log("content, ", contentRef.current);
      contentRef.current?.scrollTo(0, document.body.scrollHeight);
    }
  }, []);

  return (
    <div className="bg-active flex flex-col h-full max-h-full overflow-hidden rounded-md shadow">
      <div
        className="flex flex-row h-full overflow-y-auto justify-center w-full"
        onScroll={handleScroll}
      >
        {currentBot ? (
          fleetDetail ? (
            <div className="w-full max-w-3xl" ref={contentRef}>
              {fleetDetail.records.length > 0 ? (
                <>
                  {fleetDetail.records
                    .toSorted((a, b) => {
                      return a.insert_time > b.insert_time ? 1 : -1;
                    })
                    .map((record) => {
                      return (
                        <Record
                          key={record.id}
                          record={record}
                          refreshTrigger={() => {
                            setRefreshTrigger((prev) => prev + 1);
                            setTriggerAnswer(true);
                          }}
                        />
                      );
                    })}
                  {fleetDetail.records.at(-1)?.role === "user" && (
                    <ResponseRecord
                      key={"response-" + currentBot.id}
                      detail={fleetDetail}
                      appendRecord={appendRecord}
                      setAnswering={setAnswering}
                      triggerAnswer={triggerAnswer}
                      chatbot={currentBot}
                      atBottom={atBottom}
                    />
                  )}
                </>
              ) : (
                <div>None Records</div>
              )}
            </div>
          ) : (
            <div className={"flex flex-col h-full justify-center"}>
              <LLMChatTemplateList
                onClickTemplate={(template) => {
                  initFleetSession(template);
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
          answering ||
          !fleetDetail ||
          fleetDetail.records[fleetDetail.records.length - 1].role === "user"
        }
        appendRecord={(record) => {
          return saveSessionAndRecord(record, fleetDetail);
        }}
        sessionDetail={fleetDetail}
        botSelect={<LLMChatBotSelect />}
      />
    </div>
  );
};

export default LLMChatSessionBody;
