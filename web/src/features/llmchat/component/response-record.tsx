import RecordContent from "./record-content";
import {
  AnswerStep,
  ResponseState,
  useLLMResponse,
} from "@/hooks/use-llm-response";
import { LLMChatBot, LLMChatRecord } from "@/store/llmchat/db";
import { LLMChatSessionDetail } from "@/store/llmchat/dto";
import { useCallback, useEffect, useRef, useState } from "react";
import { v4 as uuid } from "uuid";

export const ResponseRecord = ({
  detail,
  className,
  appendRecord,
  setAnswering,
  triggerAnswer,
  chatbot,
  atBottom,
}: {
  detail: LLMChatSessionDetail;
  chatbot: LLMChatBot;
  triggerAnswer: boolean;
  className?: string;
  appendRecord: (record: LLMChatRecord) => Promise<boolean>;
  setAnswering: (flag: boolean) => void;
  atBottom: boolean;
}) => {
  if (detail.records.length <= 0) {
    return;
  }

  const [bot, setBot] = useState(chatbot);

  const handleResponse = useCallback(
    async (responseState: ResponseState) => {
      const record: LLMChatRecord = {
        id: uuid(),
        session_id: responseState.sessionId,
        content: responseState.content ?? "",
        reasoning_content: responseState.reasoningContent ?? "",
        role: "assistant",
        role_id: bot.id,
        pre_record_id: responseState.prevRecordId,
        insert_time: new Date(),
      };
      await appendRecord(record);
      return true;
    },
    [appendRecord]
  );

  const handleAfterEnd = useCallback(() => {
    setAnswering(false);
  }, [setAnswering]);

  const { answerStep, responseState, setAnswerStep } = useLLMResponse({
    bot,
    handleResponse,
    detail,
    triggerAnswer,
    afterEnd: handleAfterEnd,
  });

  const hanbleAbort =
    answerStep === AnswerStep.Answering
      ? () => setAnswerStep(AnswerStep.Abort)
      : undefined;

  const bottomDivRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (atBottom) {
      bottomDivRef.current?.scrollIntoView({ behavior: "smooth" });
    }
  }, [atBottom, responseState]);

  return (
    <>
      <RecordContent
        className={className}
        reasoningContent={responseState?.reasoningContent}
        content={responseState?.content ?? ""}
        onAbort={hanbleAbort}
        onRegenerate={() => {
          setBot(chatbot);
          setAnswerStep(AnswerStep.TriggerAnswer);
        }}
        role={"assistant-response"}
        logo={bot.svg_logo}
      />
      <div ref={bottomDivRef}></div>
    </>
  );
};
