import { LLMChatBot, LLMChatBotBodyOpenAIV1 } from "@/store/llmchat/db";
import { LLMChatSessionDetail } from "@/store/llmchat/dto";
import { fetchEventSource } from "@microsoft/fetch-event-source";
import { useCallback, useEffect, useState } from "react";
import { toast } from "sonner";

export enum AnswerStep {
  Initial,
  TriggerAnswer,
  Answering,
  TriggerHandleResponse,
  HandlingResponse,
  Done,
  Abort,
}

export interface ResponseState {
  reasoningContent: string;
  content: string;
  sessionId: string;
  prevRecordId?: string;
  abortSingal: AbortController;
}

export const useLLMResponse = ({
  detail,
  bot,
  triggerAnswer,
  handleResponse,
  onFinish,
}: {
  detail: LLMChatSessionDetail;
  bot: LLMChatBot;
  handleResponse: (record: ResponseState) => Promise<boolean>;
  triggerAnswer: boolean;
  onFinish: () => void;
}) => {
  const [answerStep, setAnswerStep] = useState(AnswerStep.Initial);
  const [responseState, setResponseState] = useState<ResponseState>();

  const doHandleResponse = useCallback(async () => {
    console.log("inner answer step:", answerStep);
    if (!responseState) {
      return;
    }
    setAnswerStep(AnswerStep.HandlingResponse);

    if (
      responseState.content.length === 0 &&
      responseState.reasoningContent.length === 0
    ) {
      console.debug("llm result is empty");
      return;
    }

    try {
      await handleResponse(responseState);
    } finally {
      setAnswerStep(AnswerStep.Done);
      setResponseState(undefined);
    }
  }, [responseState, setAnswerStep, setResponseState, handleResponse]);

  const doResponse = useCallback(() => {
    setAnswerStep(AnswerStep.Answering);
    const config = JSON.parse(bot.body) as LLMChatBotBodyOpenAIV1;

    const body = {
      model: config.model_name,
      messages: detail.records.map((r) => {
        return { role: r.role, content: r.content };
      }),
      stream: true,
    };

    const ctrl = new AbortController();
    setResponseState((prev) => {
      if (prev && prev.abortSingal) {
        prev.abortSingal.abort();
      }
      return {
        abortSingal: ctrl,
        content: "",
        prevRecordId: detail.session.id,
        sessionId: detail.session.id,
        reasoningContent: "",
      };
    });

    fetchEventSource(config.url, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${config.token}`,
      },
      body: JSON.stringify(body),
      signal: ctrl.signal,
      onmessage: (msg) => {
        const text = msg.data;
        if (text === "[DONE]") {
          setAnswerStep(AnswerStep.TriggerHandleResponse);
          return;
        }
        if (text.trim().length == 0) {
          return;
        }

        const json = JSON.parse(text);
        const choices = json.choices as Array<{
          delta: {
            content: string | null;
            reasoning_content: string | undefined | null;
          };
        }>;
        const delta = choices.at(0)?.delta;
        const content = delta?.content;
        const reasoningContent = delta?.reasoning_content;
        if (content && content.length > 0) {
          setResponseState((prev) => {
            return { ...prev!, content: prev?.content + content };
          });
        }
        if (reasoningContent && reasoningContent.length > 0) {
          setResponseState((prev) => {
            return {
              ...prev!,
              reasoningContent: prev?.reasoningContent + reasoningContent,
            };
          });
        }
      },
      onclose() {
        console.log("onclose");
        setAnswerStep((prev) => {
          return AnswerStep.TriggerHandleResponse;
        });
      },
      onerror(err) {
        throw err;
      },
    }).catch((err) => {
      console.warn("unable to fetch resources", err);
      setResponseState((prev) => {
        return {
          ...prev!,
          reasoningContent: prev?.reasoningContent + "\n\n" + err,
        };
      });

      toast.error(`Error when ask for llm result. \n ${err}`);
      setAnswerStep((prev) => {
        return AnswerStep.TriggerHandleResponse;
      });
    });
  }, [detail, bot, setAnswerStep, setResponseState]);

  const doAbort = useCallback(() => {
    console.log("try to cancel answer");

    if (
      detail.session.id !== responseState?.sessionId &&
      responseState?.abortSingal
    ) {
      console.log("cancel answer");
      responseState?.abortSingal.abort();
      setAnswerStep(AnswerStep.TriggerHandleResponse);
    }
  }, [detail, responseState, setAnswerStep]);

  useEffect(() => {
    if (answerStep === AnswerStep.TriggerHandleResponse) {
      doHandleResponse();
    } else if (answerStep === AnswerStep.TriggerAnswer) {
      doResponse();
    } else if (answerStep === AnswerStep.Abort) {
      doAbort();
    } else if (answerStep === AnswerStep.Done) {
      onFinish();
    }
  }, [answerStep, responseState]);

  if (
    detail.records.at(-1)?.role === "user" &&
    (answerStep === AnswerStep.Done || answerStep === AnswerStep.Initial) &&
    triggerAnswer
  ) {
    setAnswerStep(AnswerStep.TriggerAnswer);
  }

  return {
    answerStep,
    responseState,
    setAnswerStep,
  };
};
