import {
  LLMChatBot,
  LLMChatRecord,
  LLMChatSession,
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

const Header = () => {
  return (
    <div className=" text-white p-1 border-b h-8">
      <LLMChatBotSelect />
    </div>
  );
};

const Records = ({ records }: { records: LLMChatRecord[] }) => {
  return (
    <div className="h-full">
      {records ? (
        records
          .toSorted((a, b) => {
            return a.insert_time > b.insert_time ? 1 : -1;
          })
          .map((record) => {
            return <Record record={record} />;
          })
      ) : (
        <div>Loading</div>
      )}
    </div>
  );
};

const LLMChatSessionContainer = () => {
  const {
    currentSession,
    currentBot,
    insertSession,
    insertRecord,
    setCurrentSession,
    fetchSessionRecords,
  } = useLLMChatStore();
  const { currentNamespace } = useNamespaceStore();

  const [records, setRecords] = useState<LLMChatRecord[]>();
  const [answering, setAnswering] = useState<boolean>();

  useEffect(() => {
    if (currentSession) {
      fetchSessionRecords(currentSession).then((rsp) => {
        setRecords(rsp.records);
      });
    }
  }, [currentSession]);

  const initSession = async (template: LLMChatTemplate) => {
    let session: LLMChatSession;
    if (!currentSession) {
      session = {
        id: uuid().toString(),
        bot_id: currentBot ? currentBot.id : "1",
        template_id: template.id,
        title: "Untitled",
        namespace: currentNamespace.name,
        insert_time: new Date(),
      };
      await insertSession(session);
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

    await insertRecord(record);
    if (!currentSession) {
      setCurrentSession(session);
    }
  };

  const onSend = async (msg: string) => {
    let record: LLMChatRecord = {
      id: uuid(),
      session_id: currentSession!!.id,
      content: msg,
      role: "user",
      insert_time: new Date(),
    };

    await insertRecord(record);
    setRecords((rs) => [...rs!!, record]);
  };

  return (
    <div className="bg-panel flex flex-col h-full max-h-full overflow-hidden rounded-md shadow">
      <Header />
      <div className="h-full overflow-y-auto">
        {records ? (
          <Records records={records} />
        ) : (
          <LLMChatTemplateList
            onClickTemplate={(template) => {
              initSession(template);
            }}
          />
        )}
      </div>
      <Input onSend={onSend} disabled={answering || !currentSession} />
    </div>
  );
};

export default LLMChatSessionContainer;
