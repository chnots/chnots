import {
  LLMChatRecord,
  LLMChatSession,
  LLMChatSessionDetail,
  LLMChatSessionDetailRsp,
  LLMChatTemplate,
  useLLMChatStore,
} from "@/store/llmchat";
import LLMChatTemplateList from "../component/template-list";
import { useEffect, useState } from "react";
import { v4 as uuid } from "uuid";
import { useNamespaceStore } from "@/store/namespace";
import LLMChatSessionInput from "../component/session-input";
import { Record } from "../component/record";
import { ResponseRecord } from "../component/response-record";
import LLMChatBotSelect from "../component/bot-select";
import Header from "../component/session-header";

const LLMChatSessionContainer = () => {
  const {
    currentSession,
    currentBot,
    insertRecord,
    setCurrentSession,
    fetchSessionRecords,
    unshiftSession,
  } = useLLMChatStore();
  const { currentNamespace } = useNamespaceStore();

  const [fleetDetail, setFleetDetail] = useState<LLMChatSessionDetail>();
  const [answering, setAnswering] = useState<boolean>(false);
  const [triggerAnswer, setTriggerAnswer] = useState<boolean>(false);

  useEffect(() => {
    if (currentSession && currentSession.id !== fleetDetail?.session?.id) {
      fetchSessionRecords(currentSession).then(
        (rsp: LLMChatSessionDetailRsp) => {
          setFleetDetail({
            session: currentSession,
            records: rsp.records,
            persisted: true,
          });
        }
      );
    }
  }, [currentSession, fleetDetail]);

  useEffect(() => {
    if (answering) {
      setTriggerAnswer(false);
    }
  }, [answering]);

  const trySaveSession = async (
    detail: LLMChatSessionDetail,
    title: string
  ) => {
    if (!detail.persisted) {
      detail.session.title = title.substring(0, 400);
      await unshiftSession(detail.session);
      for (const record of detail.records) {
        await insertRecord(record);
      }
      setFleetDetail({ ...detail, persisted: true });
      setCurrentSession(detail.session);
    }
  };

  const appendRecord = async (record: LLMChatRecord) => {
    console.log("begin to insert, ", record);
    await insertRecord(record);
    setFleetDetail((prev) => {
      if (prev) {
        return {
          ...prev,
          records: [...prev?.records, record],
          persisted: true,
        };
      } else {
        return undefined;
      }
    });
    return true;
  };

  const initFleetSession = async (template: LLMChatTemplate) => {
    let session: LLMChatSession = {
      id: uuid(),
      bot_id: currentBot ? currentBot.id : "1",
      template_id: template.id,
      title: "Untitled",
      namespace: currentNamespace.name,
      insert_time: new Date(),
    };

    let record: LLMChatRecord = {
      id: uuid(),
      session_id: session.id,
      content: template.prompt,
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

  return (
    <div className="bg-panel flex flex-col h-full max-h-full overflow-hidden rounded-md shadow">
      <Header
        onAdd={() => {
          setCurrentSession(undefined);
          setFleetDetail(undefined);
        }}
      />
      <div className="flex flex-row h-full overflow-y-auto justify-center w-full">
        {currentBot ? (
          fleetDetail ? (
            <div className="w-full max-w-3xl">
              {fleetDetail.records && fleetDetail.records.length > 0 ? (
                <>
                  {fleetDetail.records
                    .toSorted((a, b) => {
                      return a.insert_time > b.insert_time ? 1 : -1;
                    })
                    .map((record) => {
                      return <Record key={record.id} record={record} />;
                    })}
                  {fleetDetail.records.at(-1)?.role === "user" && (
                    <ResponseRecord
                      detail={fleetDetail}
                      appendRecord={appendRecord}
                      setAnswering={setAnswering}
                      triggerAnswer={triggerAnswer}
                      bot={currentBot}
                    />
                  )}
                </>
              ) : (
                <div>None Records</div>
              )}
            </div>
          ) : (
            <LLMChatTemplateList
              onClickTemplate={(template) => {
                initFleetSession(template);
              }}
            />
          )
        ) : (
          <div>Please add a bot</div>
        )}
      </div>
      <LLMChatSessionInput
        disabled={answering || !fleetDetail}
        appendRecord={(record) => {
          return saveSessionAndRecord(record, fleetDetail);
        }}
        sessionDetail={fleetDetail}
      />
    </div>
  );
};

export default LLMChatSessionContainer;
