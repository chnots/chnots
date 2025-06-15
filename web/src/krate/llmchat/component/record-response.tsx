import {
  ResponseCtl,
  ResponseStep,
  ResponseState,
  useLLMResponse,
} from "@/hooks/use-llm-response";
import { LLMChatBot, LLMChatRecord } from "@/krate/llmchat/po";
import { LLMChatContainerSession } from "@/krate/llmchat/dto";
import { useEffect, useRef } from "react";
import RecordAssistant from "./record-assistant";
import { llmchatRecordInsert } from "@/krate/llmchat/service";
import Icon from "@/common/component/icon";
import { Button as KButton } from "@/common/component/ui/button";
import { genTID } from "@/lib/id_util";

export const RecordAnswering = ({
  containerSession,
  triggerAnswer,
  bot,
  onScrollToEnd,
  onRegenerate,
  onEnd,
  onSetResponsing,
}: {
  containerSession: LLMChatContainerSession;
  bot: LLMChatBot;
  triggerAnswer: boolean;
  onScrollToEnd?: () => void;
  onRegenerate?: () => void;
  onSetResponsing?: (flag: boolean) => void;
  onEnd: (record: LLMChatRecord) => void;
}) => {
  const savedStateRef = useRef<ResponseState>(undefined);

  if (containerSession.records.length <= 0) {
    return;
  }

  const { response, setAnswerCtl } = useLLMResponse({
    bot,
    detail: containerSession,
  });

  const onAbort =
    response.step === ResponseStep.Answering
      ? () => setAnswerCtl(ResponseCtl.Abort)
      : undefined;

  useEffect(() => {
    if (onSetResponsing) {
      onSetResponsing(
        (response && response.step !== ResponseStep.End) ?? false
      );
    }
  }, [onSetResponsing, response]);

  useEffect(() => {
    if (triggerAnswer) {
      setAnswerCtl(ResponseCtl.Trigger);
    }

    return () => {
      setAnswerCtl(ResponseCtl.Abort);
    };
  }, [setAnswerCtl, triggerAnswer]);

  useEffect(() => {
    return () => {
      if (savedStateRef.current) {
        const response = savedStateRef.current;
        const record: LLMChatRecord = {
          tid: response.tid,
          session_id: response.sessionId,
          content: response.content,
          reasoning_content: response.reasoningContent,
          role: "assistant",
          role_id: response.roleId,
          pre_record_id: response.prevRecordId,
        };
        llmchatRecordInsert(record);
      }
    };
  }, []);

  useEffect(() => {
    const buildRecord = (responseState: ResponseState) => {
      const record: LLMChatRecord = {
        tid: responseState.tid,
        session_id: responseState.sessionId,
        content: responseState.content,
        reasoning_content: responseState.reasoningContent,
        role: "assistant",
        role_id: responseState.roleId,
        pre_record_id: responseState.prevRecordId,
      };

      return record;
    };

    if (response && response.step == ResponseStep.End) {
      onEnd(buildRecord(response));
      savedStateRef.current = undefined;
    } else {
      savedStateRef.current = response;
    }
    if (onScrollToEnd) {
      onScrollToEnd();
    }
  }, [response, savedStateRef, onScrollToEnd]);

  return (
    <>
      <RecordAssistant
        logo={bot.svg_logo}
        role_id={bot.tid}
        onRegenerate={onRegenerate}
        tid={genTID()}
        session_id={response.sessionId}
        content={response.content}
        reasoning_content={response.reasoningContent}
        role={"response-assistant"}
      />
      <div className="flex justify-center">
        <KButton
          onClick={onAbort}
          className="p-1 rounded-full hover:bg-gray-200 focus:outline-none transition-colors flex mb-6"
          aria-label="Abort"
          tabIndex={0}
        >
          <Icon.Square className="h-4 w-4 text-gray-700" />
          <span>Stop Generate</span>
        </KButton>
      </div>
    </>
  );
};
