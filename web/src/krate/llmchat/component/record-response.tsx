import { useChat } from "@ai-sdk/react";
import type { UIMessage } from "ai";
import { Square } from "lucide-react";
import { useEffect, useMemo, useRef } from "react";
import { Button as KButton } from "@/common/component/ui/button";
import type { LLMChatBot } from "@/krate/llmchat/po";
import { llmchatRecordCommit } from "@/krate/llmchat/service";
import { genTID, type TID } from "@/lib/id_util";
import { partsToContentBlocks, recordVOToUIMessages } from "../message-adapter";
import { createTransport } from "../transport";
import type { LLMChatRecordVO } from "../vo";
import RecordAssistant from "./record-assistant";
import { useLLMChatComStore } from "./session";

const RecordAnsweringInner = ({
  bot,
  session,
  records,
  onScrollToEnd,
}: {
  bot: LLMChatBot;
  session: { otid: TID };
  records: LLMChatRecordVO[];
  onScrollToEnd?: () => void;
}) => {
  const { setResponsing, appendRecord } = useLLMChatComStore((store) => {
    return {
      setResponsing: store.setResponsing,
      appendRecord: store.appendRecord,
    };
  });

  const initialMessages = useMemo(
    () => recordVOToUIMessages(records) as UIMessage[],
    [records],
  );
  const transport = useMemo(() => createTransport(bot.body), [bot.body]);
  const assistantOtid = useRef<TID>(genTID());

  const { messages, stop, status } = useChat({
    transport: transport as never,
    messages: initialMessages,
    sendAutomaticallyWhen: ({ messages: msgs }) => {
      const last = msgs.at(-1);
      return last?.role === "user";
    },
    onFinish: ({ message, isAbort }) => {
      const prevRecordOtid = records.at(-1)?.otid;
      const record: LLMChatRecordVO = {
        otid: assistantOtid.current,
        session_otid: session.otid,
        pre_record_otid: prevRecordOtid,
        content: partsToContentBlocks(message.parts),
        role: "assistant",
        role_id: bot.otid,
        tid: genTID(),
      };

      if (!isAbort) {
        appendRecord(record);
      } else {
        llmchatRecordCommit(record);
      }
    },
  });

  useEffect(() => {
    setResponsing(status === "submitted" || status === "streaming");
  }, [status, setResponsing]);

  useEffect(() => {
    return () => {
      stop();
    };
  }, [stop]);

  useEffect(() => {
    onScrollToEnd?.();
  }, [messages, onScrollToEnd]);

  const lastAssistantMsg = [...messages]
    .reverse()
    .find((m) => m.role === "assistant");
  if (!lastAssistantMsg) {
    return null;
  }

  return (
    <>
      <RecordAssistant
        logo={bot.svg_logo}
        role_id={bot.otid}
        otid={assistantOtid.current}
        session_otid={session.otid}
        content={partsToContentBlocks(lastAssistantMsg.parts)}
        tid={genTID()}
        timestamp={new Date(assistantOtid.current / 1e3).toISOString()}
        viewMode={false}
        isAnimating={true}
      />
      {(status === "submitted" || status === "streaming") && (
        <div className="flex justify-center">
          <KButton
            onClick={() => stop()}
            className="p-1 rounded-full hover:bg-gray-200 focus:outline-none transition-colors flex mb-6"
            aria-label="Abort"
            tabIndex={0}
          >
            <Square className="h-4 w-4 text-gray-700" />
            <span>Stop Generate</span>
          </KButton>
        </div>
      )}
    </>
  );
};

export const RecordAnswering = ({
  bot,
  onScrollToEnd,
}: {
  bot: LLMChatBot;
  onScrollToEnd?: () => void;
}) => {
  const { records, session } = useLLMChatComStore((store) => {
    return {
      records: store.records,
      session: store.session,
    };
  });

  if (!session || !records || records.length <= 0) {
    return null;
  }

  return (
    <RecordAnsweringInner
      bot={bot}
      session={session}
      records={records}
      onScrollToEnd={onScrollToEnd}
    />
  );
};
