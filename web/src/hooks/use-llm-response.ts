import { streamText } from "ai";
import { useCallback, useEffect, useRef, useState } from "react";
import { toast } from "sonner";

import type { LLMChatBot, LLMChatSession } from "@/krate/llmchat/po";
import {
  createLLMProvider,
  parseBotBody,
} from "@/krate/llmchat/provider-manager";
import type { LLMChatRecordVO } from "@/krate/llmchat/vo";
import { genTID, type TID } from "@/lib/id_util";

export enum ResponseStep {
  Initial = "init",
  Answering = "ans",
  Answered = "fia",
  Aborted = "abt",
  Error = "err",
  End = "end",
}

export enum ResponseCtl {
  Trigger = "tri",
  Abort = "abt",
}

export type ResponseState = {
  tid: TID;
  step: ResponseStep;
  sessionId: TID;
  prevRecordId?: TID;
  roleId: TID;
  reasoningContent: string;
  content: string;
};

const emptyResponse = (session: LLMChatSession, bot: LLMChatBot) => {
  return {
    tid: genTID(),
    step: ResponseStep.Initial,
    prevRecordId: session.otid,
    sessionId: session.otid,
    roleId: bot.otid,
    content: "",
    reasoningContent: "",
  };
};

export const useLLMResponse = ({
  session,
  records,
  bot,
}: {
  session: LLMChatSession;
  records: LLMChatRecordVO[];
  bot: LLMChatBot;
}) => {
  const [answerCtl, setAnswerCtl] = useState<ResponseCtl | undefined>(
    undefined,
  );
  const [responseState, setResponseState] = useState<ResponseState>(
    emptyResponse(session, bot),
  );
  const abortControllerRef = useRef<AbortController | null>(null);

  const doPostResponse = useCallback(async () => {
    if (
      responseState.content.length === 0 &&
      responseState.reasoningContent.length === 0
    ) {
      return;
    }

    let ended = false;
    if (responseState.reasoningContent.length === 0) {
      const content = responseState.content;
      const thinkStart = content.indexOf("<think>");
      const thinkEnd = content.indexOf("</think>");
      if (thinkStart >= 0) {
        if (thinkEnd > 0) {
          setResponseState((prev) => {
            return {
              ...prev,
              step: ResponseStep.End,
              content: content.substring(thinkEnd + 8),
              reasoningContent: content.substring(thinkStart + 7, thinkEnd),
            };
          });
          ended = true;
        } else {
          setResponseState((prev) => {
            return {
              ...prev,
              step: ResponseStep.End,
              content: "",
              reasoningContent: content.substring(thinkStart + 7),
            };
          });
          ended = true;
        }
      }
    }

    if (!ended) {
      setResponseState((prev) => {
        return { ...prev, step: ResponseStep.End };
      });
    }
  }, [responseState]);

  const doAbort = useCallback(() => {
    abortControllerRef.current?.abort();
    abortControllerRef.current = null;
    setResponseState((prev) => {
      return { ...prev, step: ResponseStep.Aborted };
    });
  }, []);

  useEffect(() => {
    return () => {
      doAbort();
    };
  }, [doAbort]);

  const doResponse = useCallback(() => {
    setResponseState((prev) => {
      return { ...prev, step: ResponseStep.Answering };
    });

    abortControllerRef.current?.abort();
    const abortController = new AbortController();
    abortControllerRef.current = abortController;

    const config = parseBotBody(bot.body);
    const model = createLLMProvider(config);

    const messages = records.map((r) => {
      let content = r.body;
      if (r.thinking) {
        content = `<think${r.thinking}</think${r.body}`;
      }
      return {
        role: r.role as "system" | "user" | "assistant",
        content,
      };
    });

    const runStream = async () => {
      try {
        const result = streamText({
          model,
          messages,
          abortSignal: abortController.signal,
          onError: (error) => {
            toast.error(`LLM Error: ${error.error}`);
            setResponseState((prev) => {
              return {
                ...prev,
                step: ResponseStep.Error,
                reasoningContent: prev.reasoningContent + String(error.error),
              };
            });
          },
        });

        for await (const part of result.fullStream) {
          if (abortController.signal.aborted) {
            break;
          }

          switch (part.type) {
            case "text-delta": {
              setResponseState((prev) => {
                return {
                  ...prev,
                  content: prev.content + part.text,
                };
              });
              break;
            }
            case "reasoning-delta": {
              setResponseState((prev) => {
                return {
                  ...prev,
                  reasoningContent: prev.reasoningContent + part.text,
                };
              });
              break;
            }
            case "error": {
              setResponseState((prev) => {
                return {
                  ...prev,
                  step: ResponseStep.Error,
                  reasoningContent: prev.reasoningContent + String(part.error),
                };
              });
              break;
            }
          }
        }

        if (!abortController.signal.aborted) {
          setResponseState((prev) => {
            return {
              ...prev,
              step: ResponseStep.Answered,
            };
          });
        }
      } catch (err) {
        if (!abortController.signal.aborted) {
          toast.error(`Error when ask for llm result. \n ${err}`);
          setResponseState((prev) => {
            return {
              ...prev,
              step: ResponseStep.Error,
              reasoningContent: prev.reasoningContent,
            };
          });
        }
      }
    };

    runStream();
  }, [bot.body, records]);

  useEffect(() => {
    if (
      answerCtl === ResponseCtl.Abort &&
      responseState.step !== ResponseStep.End
    ) {
      doAbort();
    } else if (
      answerCtl === ResponseCtl.Trigger &&
      responseState.step !== ResponseStep.Answering
    ) {
      setResponseState(emptyResponse(session, bot));
      doResponse();
    }
    setAnswerCtl(undefined);
  }, [session, bot, answerCtl, responseState, doAbort, doResponse]);

  useEffect(() => {
    if (
      [
        ResponseStep.Aborted,
        ResponseStep.Answered,
        ResponseStep.Error,
      ].includes(responseState.step)
    ) {
      doPostResponse();
    }
  }, [responseState, doPostResponse]);

  return {
    response: responseState,
    setAnswerCtl,
  };
};
