import {
  LLMChatBot,
  LLMChatRecord,
  LLMChatSessionDetail,
} from "@/store/llmchat";
import RecordContent from "./record-content";
import {
  AnswerStep,
  ResponseState,
  useLLMResponse,
} from "@/hooks/use-llm-response";
import { useCallback } from "react";
import { v4 as uuid } from "uuid";
export const ResponseRecord = ({
  detail,
  className,
  appendRecord,
  setAnswering,
  triggerAnswer,
  bot,
}: {
  detail: LLMChatSessionDetail;
  bot: LLMChatBot;
  triggerAnswer: boolean;
  className?: string;
  appendRecord: (record: LLMChatRecord) => Promise<boolean>;
  setAnswering: (flag: boolean) => void;
}) => {
  if (detail.records.length <= 0) {
    return;
  }
  const handleResponse = useCallback(
    async (responseState: ResponseState) => {
      const record: LLMChatRecord = {
        id: uuid(),
        session_id: responseState.sessionId,
        content: responseState.answer ?? "",
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

  return (
    <RecordContent
      className={className}
      content={responseState?.answer ?? ""}
      onAbort={hanbleAbort}
      onRegenerate={() => {
        setAnswerStep(AnswerStep.TriggerAnswer);
      }}
      role={"assistant-response"}
      logo={bot.svg_logo}
    />
  );
};
