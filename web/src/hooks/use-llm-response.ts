import { streamText } from "ai";
import { useCallback, useEffect, useRef, useState } from "react";
import { toast } from "sonner";

import type {
  ContentBlock,
  LLMChatBot,
  LLMChatSession,
} from "@/krate/llmchat/po";
import { buildContentBlocks, getBlockContent } from "@/krate/llmchat/po";
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
  contentBlocks: ContentBlock[];
};

const updateBlock = (
  blocks: ContentBlock[],
  type: ContentBlock["type"],
  data: string,
): ContentBlock[] => {
  const idx = blocks.findIndex((b) => b.type === type);
  if (idx >= 0) {
    const newBlocks = [...blocks];
    newBlocks[idx] = { type, data: newBlocks[idx].data + data };
    return newBlocks;
  }
  if (type === "content") {
    return [...blocks, { type, data }];
  }
  return [{ type, data }, ...blocks];
};

const setBlock = (
  blocks: ContentBlock[],
  type: ContentBlock["type"],
  data: string,
): ContentBlock[] => {
  const idx = blocks.findIndex((b) => b.type === type);
  if (idx >= 0) {
    const newBlocks = [...blocks];
    newBlocks[idx] = { type, data };
    return newBlocks;
  }
  if (type === "content") {
    return [...blocks, { type, data }];
  }
  return [{ type, data }, ...blocks];
};

const emptyResponse = (
  session: LLMChatSession,
  bot: LLMChatBot,
  records: LLMChatRecordVO[],
) => {
  return {
    tid: genTID(),
    step: ResponseStep.Initial,
    prevRecordId: records.at(-1)?.otid ?? undefined,
    sessionId: session.otid,
    roleId: bot.otid,
    contentBlocks: [],
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
    emptyResponse(session, bot, records),
  );
  const abortControllerRef = useRef<AbortController | null>(null);

  const doPostResponse = useCallback(() => {
    const blocks = responseState.contentBlocks;
    if (blocks.length === 0) {
      return;
    }

    const contentBlock = blocks.find((b) => b.type === "content");
    const thinkingBlock = blocks.find((b) => b.type === "thinking");

    let ended = false;
    if (!thinkingBlock) {
      const content = contentBlock?.data ?? "";
      const thinkStart = content.indexOf("aisse");
      const thinkEnd = content.indexOf(" асс");
      if (thinkStart >= 0) {
        if (thinkEnd > 0) {
          setResponseState((prev) => {
            return {
              ...prev,
              step: ResponseStep.End,
              contentBlocks: buildContentBlocks(
                content.substring(thinkEnd + 8),
                content.substring(thinkStart + 7, thinkEnd),
              ),
            };
          });
          ended = true;
        } else {
          setResponseState((prev) => {
            return {
              ...prev,
              step: ResponseStep.End,
              contentBlocks: buildContentBlocks(
                "",
                content.substring(thinkStart + 7),
              ),
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
      const body = getBlockContent(r.content, "content");
      const thinking = getBlockContent(r.content, "thinking");
      let content = body;
      if (thinking) {
        content = `<think${thinking}</think${body}`;
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
                contentBlocks: updateBlock(
                  prev.contentBlocks,
                  "error",
                  String(error.error),
                ),
              };
            });
          },
        });

        for await (const part of result.fullStream) {
          if (abortController.signal.aborted) {
            return;
          }

          switch (part.type) {
            case "text-delta": {
              setResponseState((prev) => {
                return {
                  ...prev,
                  contentBlocks: updateBlock(
                    prev.contentBlocks,
                    "content",
                    part.text,
                  ),
                };
              });
              break;
            }
            case "reasoning-delta": {
              setResponseState((prev) => {
                return {
                  ...prev,
                  contentBlocks: updateBlock(
                    prev.contentBlocks,
                    "thinking",
                    part.text,
                  ),
                };
              });
              break;
            }
            case "error": {
              setResponseState((prev) => {
                return {
                  ...prev,
                  step: ResponseStep.Error,
                  contentBlocks: updateBlock(
                    prev.contentBlocks,
                    "error",
                    String(part.error),
                  ),
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
      setResponseState(emptyResponse(session, bot, records));
      doResponse();
    }
    setAnswerCtl(undefined);
  }, [session, bot, records, answerCtl, responseState, doAbort, doResponse]);

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
