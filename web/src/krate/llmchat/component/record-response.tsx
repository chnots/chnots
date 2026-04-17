import { useChat } from "@ai-sdk/react";
import type { UIMessage } from "ai";
import { isToolUIPart } from "ai";
import { useEffect, useMemo, useRef } from "react";
import type { LLMChatBot, RecordContent } from "@/krate/llmchat/po";
import { buildRecordContentFromParts } from "@/krate/llmchat/po";
import { genTID, type TID } from "@/lib/id_util";
import { partsToContentBlocks, recordVOToUIMessages } from "../message-adapter";
import type { LLMChatMessageMetadata } from "../transport";
import { createTransport } from "../transport";
import type { LLMChatRecordVO } from "../vo";
import RecordAssistant from "./record-assistant";
import { useLLMChatComStore } from "./session";
import { ToolCallBlock } from "./tool-call-block";

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
  const { appendRecord, setResponsing, setStopGenerating } = useLLMChatComStore(
    (store) => {
      return {
        appendRecord: store.appendRecord,
        setResponsing: store.setResponsing,
        setStopGenerating: store.setStopGenerating,
      };
    },
  );

  const initialMessages = useMemo(
    () => recordVOToUIMessages(records) as UIMessage[],
    [records],
  );
  const transport = useMemo(() => createTransport(bot.body), [bot.body]);
  const assistantOtid = useRef<TID>(genTID());
  const initialMsgCountRef = useRef(initialMessages.length);

  const { messages, sendMessage, stop, status } = useChat({
    transport: transport as never,
    messages: initialMessages,
    sendAutomaticallyWhen: ({ messages: msgs }) => {
      const last = msgs.at(-1);
      return last?.role === "user";
    },
    onFinish: ({ message }) => {
      const prevRecordOtid = records.at(-1)?.otid;
      const metadata = message.metadata as LLMChatMessageMetadata | undefined;
      const record: LLMChatRecordVO = {
        otid: assistantOtid.current,
        session_otid: session.otid,
        pre_record_otid: prevRecordOtid,
        content: buildRecordContentFromParts(
          partsToContentBlocks(message.parts),
          metadata?.usage,
        ),
        role: "assistant",
        role_id: bot.otid,
        tid: genTID(),
      };

      appendRecord(record);
      setResponsing(false);
    },
  });

  const sentRef = useRef(false);
  useEffect(() => {
    if (!sentRef.current) {
      sentRef.current = true;
      sendMessage();
    }
  }, [sendMessage]);

  useEffect(() => {
    setStopGenerating(stop);
    return () => {
      setStopGenerating(null);
    };
  }, [stop, setStopGenerating]);

  useEffect(() => {
    onScrollToEnd?.();
  }, [messages, onScrollToEnd]);

  const lastAssistantMsg = [...messages]
    .slice(initialMsgCountRef.current)
    .reverse()
    .find((m) => m.role === "assistant");
  const isResponding = status === "submitted" || status === "streaming";

  if (!lastAssistantMsg && !isResponding) {
    return null;
  }

  const usage = (
    lastAssistantMsg?.metadata as LLMChatMessageMetadata | undefined
  )?.usage;

  const assistantContent: RecordContent = lastAssistantMsg
    ? {
        usage: usage ?? {},
        parts: partsToContentBlocks(lastAssistantMsg.parts),
      }
    : { usage: {}, parts: [] };
  const toolParts = lastAssistantMsg
    ? lastAssistantMsg.parts.filter((p) => isToolUIPart(p))
    : [];

  return (
    <>
      <RecordAssistant
        logo={bot.svg_logo}
        role_id={bot.otid}
        otid={assistantOtid.current}
        session_otid={session.otid}
        content={assistantContent}
        tid={genTID()}
        timestamp={new Date(assistantOtid.current / 1e3).toISOString()}
        viewMode={false}
        isAnimating={true}
      />
      {toolParts.length > 0 && (
        <div className="mx-4">
          {toolParts.map((part, i) => {
            const tp = part as {
              type: string;
              toolCallId: string;
              toolName?: string;
              state: string;
              input?: unknown;
              output?: unknown;
              errorText?: string;
            };
            return (
              <ToolCallBlock
                key={tp.toolCallId ?? i}
                toolName={tp.toolName ?? tp.type.replace("tool-", "")}
                toolCallId={tp.toolCallId}
                state={
                  tp.state as
                    | "input-streaming"
                    | "input-available"
                    | "approval-requested"
                    | "approval-responded"
                    | "output-available"
                    | "output-error"
                    | "output-denied"
                }
                input={tp.input}
                output={tp.output}
                errorText={tp.errorText}
              />
            );
          })}
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
