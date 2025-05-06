import {
  ResponseCtl,
  ResponseStep,
  ResponseState,
  useLLMResponse,
} from "@/hooks/use-llm-response";
import { LLMChatBot, LLMChatRecord } from "@/store/llmchat/db";
import { LLMChatContainerSession } from "@/store/llmchat/dto";
import React, { useEffect, useRef, useState } from "react";
import RecordAssistant from "./record-assistant";
import { llmchatRecordInsert } from "@/store/llmchat/service";

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
    console.log("创建新的 RecordAnswering ");
    return () => {
      if (savedStateRef.current) {
        const response = savedStateRef.current;
        const record: LLMChatRecord = {
          id: response.id,
          session_id: response.sessionId,
          content: response.content,
          reasoning_content: response.reasoningContent,
          role: "assistant",
          role_id: response.roleId,
          pre_record_id: response.prevRecordId,
          insert_time: new Date(),
        };
        llmchatRecordInsert(record);
      }
    };
  }, []);

  useEffect(() => {
    const buildRecord = (responseState: ResponseState) => {
      const record: LLMChatRecord = {
        id: responseState.id,
        session_id: responseState.sessionId,
        content: responseState.content,
        reasoning_content: responseState.reasoningContent,
        role: "assistant",
        role_id: responseState.roleId,
        pre_record_id: responseState.prevRecordId,
        insert_time: new Date(),
      };

      return record;
    };

    if (response && response.step == ResponseStep.End) {
      onEnd(buildRecord(response));
      savedStateRef.current = undefined;
    } else {
      savedStateRef.current = response;
    }
  }, [response, savedStateRef]);



  return (
    <>
      <RecordAssistant
        logo={bot.svg_logo}
        role_id={bot.id}
        onAbort={onAbort}
        onRegenerate={onRegenerate}
        id={response.sessionId + "-response"}
        session_id={response.sessionId}
        content={response.content}
        reasoning_content={response.reasoningContent}
        role={"response-assistant"}
        insert_time={new Date()}
      />
    </>
  );
};
