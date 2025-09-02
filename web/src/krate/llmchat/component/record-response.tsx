import {
  ResponseCtl,
  ResponseStep,
  ResponseState,
  useLLMResponse,
} from "@/hooks/use-llm-response";
import { LLMChatBot, LLMChatRecord } from "@/krate/llmchat/po";
import { useEffect, useRef } from "react";
import RecordAssistant from "./record-assistant";
import { llmchatRecordInsert } from "@/krate/llmchat/service";
import Icon from "@/common/component/icon";
import { Button as KButton } from "@/common/component/ui/button";
import { genTID, TID } from "@/lib/id_util";
import { useLLMChatComStore } from "./llm-chat-session";

export const RecordAnswering = ({
  bot,
  onScrollToEnd,
}: {
  bot: LLMChatBot;
  onScrollToEnd?: () => void;
}) => {
  const { records, session, setResponsing, appendRecord, answering } =
    useLLMChatComStore((store) => {
      return {
        records: store.records,
        session: store.session,
        setResponsing: store.setResponsing,
        appendRecord: store.appendRecord,
        answering: store.responsing,
      };
    });
  const responseStateRef = useRef<ResponseState>(undefined);
  const otid = useRef<TID>(genTID());

  if (!session || !records || records.length <= 0) {
    return;
  }

  const { response, setAnswerCtl } = useLLMResponse({
    bot,
    session,
    records,
  });

  const onAbort =
    response.step === ResponseStep.Answering
      ? () => setAnswerCtl(ResponseCtl.Abort)
      : undefined;

  useEffect(() => {
    if (setResponsing) {
      setResponsing((response && response.step !== ResponseStep.End) ?? false);
    }
  }, [response]);

  useEffect(() => {
    if (answering) {
      setAnswerCtl(ResponseCtl.Trigger);
    }

    return () => {
      setAnswerCtl(ResponseCtl.Abort);
    };
  }, [answering]);

  useEffect(() => {
    return () => {
      if (responseStateRef.current) {
        const response = responseStateRef.current;
        const record: LLMChatRecord = {
          otid: response.tid,
          session_otid: response.sessionId,
          content: response.content,
          reasoning_content: response.reasoningContent,
          role: "assistant",
          role_id: response.roleId,
          pre_record_otid: response.prevRecordId,
          tid: genTID(),
        };
        llmchatRecordInsert(record);
      }
    };
  }, []);

  useEffect(() => {
    const buildRecord = (responseState: ResponseState) => {
      const record: LLMChatRecord = {
        otid: responseState.tid,
        session_otid: responseState.sessionId,
        content: responseState.content,
        reasoning_content: responseState.reasoningContent,
        role: "assistant",
        role_id: responseState.roleId,
        pre_record_otid: responseState.prevRecordId,
        tid: genTID(),
      };

      return record;
    };

    if (response && response.step == ResponseStep.End) {
      appendRecord(buildRecord(response));
      responseStateRef.current = undefined;
    } else {
      responseStateRef.current = response;
    }
    if (onScrollToEnd) {
      onScrollToEnd();
    }
  }, [response, responseStateRef, onScrollToEnd]);

  return (
    <>
      <RecordAssistant
        logo={bot.svg_logo}
        role_id={bot.otid}
        otid={otid.current}
        session_otid={response.sessionId}
        content={response.content}
        reasoning_content={response.reasoningContent}
        role={"response-assistant"}
        tid={genTID()}
        timestamp={new Date(otid.current / 1e3).toISOString()}
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
