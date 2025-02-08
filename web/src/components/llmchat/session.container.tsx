import {
  LLMChatRecord,
  LLMChatSession,
  LLMChatSessionDetailRsp,
  LLMChatTemplate,
  useLLMChatStore,
} from "@/store/llmchat";
import LLMChatTemplateList from "./template-list.component";
import { useEffect, useState } from "react";
import { v4 as uuid } from "uuid";
import { useNamespaceStore } from "@/store/namespace";
import LLMChatBotSelect from "./bot-select.component";
import Input from "./session-input.component";
import { Record } from "./record.component";
import { ResponseRecord } from "./response-record.component";

const Header = () => {
  return (
    <div className="p-1 border-b">
      <LLMChatBotSelect />
    </div>
  );
};

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

  const [records, setRecords] = useState<LLMChatRecord[]>();
  const [answering, setAnswering] = useState<boolean>();
  const [notPersistedId, setNotPersistedId] = useState<string>("");
  const [triggerAnswer, setTriggerAnswer] = useState<boolean>(false);

  useEffect(() => {
    if (currentSession && currentSession.id !== notPersistedId) {
      fetchSessionRecords(currentSession).then(
        (rsp: LLMChatSessionDetailRsp) => {
          setRecords(rsp.records);
        }
      );
    }
  }, [currentSession, notPersistedId]);

  useEffect(() => {
    if (answering) {
      setTriggerAnswer(false);
    }
  }, [answering]);

  const appendRecord = async (record: LLMChatRecord) => {
    console.log("begin to insert, ", record);
    await insertRecord(record);
    setRecords((rs) => [...rs!!, record]);
  };

  const initSession = async (template: LLMChatTemplate) => {
    let session: LLMChatSession;
    if (!currentSession) {
      const id = uuid().toString();
      session = {
        id: id,
        bot_id: currentBot ? currentBot.id : "1",
        template_id: template.id,
        title: "Untitled",
        namespace: currentNamespace.name,
        insert_time: new Date(),
      };
      setNotPersistedId(id);
    } else {
      session = currentSession;
    }

    let record: LLMChatRecord = {
      id: uuid(),
      session_id: session.id,
      content: template.prompt,
      role: "system",
      insert_time: new Date(),
    };

    setRecords([record]);

    if (!currentSession) {
      setCurrentSession(session);
    }
  };

  const onSendUserMsg = async (msg: string) => {
    let record: LLMChatRecord = {
      id: uuid(),
      session_id: currentSession!!.id,
      pre_record_id: records?.at(-1)?.id,
      content: msg,
      role: "user",
      insert_time: new Date(),
    };
    if (
      notPersistedId === currentSession?.id &&
      records &&
      records?.length > 0
    ) {
      const tmpCurSession = { ...currentSession, title: record.content };
      setCurrentSession(tmpCurSession);
      await unshiftSession(tmpCurSession);

      for (const r of records) {
        await insertRecord(r);
      }

      setNotPersistedId("");
    }

    await appendRecord(record);
    setTriggerAnswer(true);
  };

  return (
    <div className="bg-panel flex flex-col h-full max-h-full overflow-hidden rounded-md shadow">
      <Header />
      <div className="h-full overflow-y-auto">
        {currentBot ? (
          records && currentSession ? (
            <div className="h-full">
              {records && records.length > 0 ? (
                <>
                  {records
                    .toSorted((a, b) => {
                      return a.insert_time > b.insert_time ? 1 : -1;
                    })
                    .map((record) => {
                      return <Record key={record.id} record={record} />;
                    })}
                  {records.at(-1)?.role === "user" ? (
                    <ResponseRecord
                      session={currentSession}
                      records={records}
                      appendRecord={appendRecord}
                      setAnswering={setAnswering}
                      triggerAnswer={triggerAnswer}
                    />
                  ) : (
                    <></>
                  )}
                </>
              ) : (
                <div>None Records</div>
              )}
            </div>
          ) : (
            <LLMChatTemplateList
              onClickTemplate={(template) => {
                initSession(template);
              }}
            />
          )
        ) : (
          <div>Please add a bot</div>
        )}
      </div>
      <Input onSend={onSendUserMsg} disabled={answering || !currentSession} />
    </div>
  );
};

export default LLMChatSessionContainer;
