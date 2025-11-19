import { useCallback, useEffect, useRef, useState } from 'react';
import { fetchEventSource } from '@microsoft/fetch-event-source';
import { toast } from 'sonner';

import type {
  LLMChatBot,
  LLMChatBotBodyOpenAIV1,
  LLMChatRecord,
  LLMChatSession,
} from '@/krate/llmchat/po';
import { genTID, type TID } from '@/lib/id_util';

export enum ResponseStep {
  Initial = 'init',
  Answering = 'ans',
  Answered = 'fia',
  Aborted = 'abt',
  Error = 'err',
  End = 'end',
}

export enum ResponseCtl {
  Trigger = 'tri',
  Abort = 'abt',
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
    content: '',
    reasoningContent: '',
  };
};

export const useLLMResponse = ({
  session,
  records,
  bot,
}: {
  session: LLMChatSession;
  records: LLMChatRecord[];
  bot: LLMChatBot;
}) => {
  const [answerCtl, setAnswerCtl] = useState<ResponseCtl | undefined>(undefined);
  const [responseState, setResponseState] = useState<ResponseState>(emptyResponse(session, bot));
  const abortSignal = useRef<AbortController>(null);

  useEffect(() => {
    return () => {
      doAbort();
    };
  }, []);

  const doPostResponse = useCallback(async () => {
    if (responseState.content.length === 0 && responseState.reasoningContent.length === 0) {
      console.debug('llm result is empty');
      return;
    }

    // Work for some local llm service, like Ollama, VLLM
    let ended = false;
    if (responseState.reasoningContent.length === 0) {
      const content = responseState.content;
      const thinkStart = content.indexOf('<think>');
      const thinkEnd = content.indexOf('</think>');
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
              content: '',
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
  }, [responseState, setResponseState]);

  const doAbort = useCallback(() => {
    abortSignal.current?.abort();
    setResponseState((prev) => {
      return { ...prev, step: ResponseStep.Aborted };
    });
  }, [abortSignal, setResponseState]);

  const doResponse = useCallback(() => {
    setResponseState((prev) => {
      return { ...prev, step: ResponseStep.Answering };
    });
    const ctrl = new AbortController();
    abortSignal.current?.abort();
    abortSignal.current = ctrl;

    const config = JSON.parse(bot.body) as LLMChatBotBodyOpenAIV1;
    const body = {
      model: config.model_name,
      messages: records.map((r) => {
        return { role: r.role, content: r.content };
      }),
      stream: true,
    };

    fetchEventSource(config.url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${config.token}`,
      },
      body: JSON.stringify(body),
      signal: ctrl.signal,
      openWhenHidden: true,
      onmessage: (msg) => {
        const text = msg.data;
        if (text === '[DONE]') {
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

        setResponseState((prev) => {
          return {
            ...prev,
            content: content ? prev.content + content : prev.content,
            reasoningContent: reasoningContent
              ? prev.reasoningContent + reasoningContent
              : prev.reasoningContent,
          };
        });
      },
      onclose() {
        console.log('onclose');
        setResponseState((prev) => {
          return {
            ...prev,
            step: ResponseStep.Answered,
          };
        });
      },
      onerror(err) {
        throw err;
      },
    }).catch((err) => {
      console.warn('unable to fetch kfiles', err);
      setResponseState((prev) => {
        return {
          ...prev,
          step: ResponseStep.Error,
          reasoningContent: prev.reasoningContent + err,
        };
      });
      toast.error(`Error when ask for llm result. \n ${err}`);
    });
  }, [setResponseState, abortSignal]);

  useEffect(() => {
    if (answerCtl === ResponseCtl.Abort && responseState.step !== ResponseStep.End) {
      doAbort();
    } else if (answerCtl === ResponseCtl.Trigger && responseState.step !== ResponseStep.Answering) {
      setResponseState(emptyResponse(session, bot));
      doResponse();
    }
    setAnswerCtl(undefined);
  }, [session, bot, answerCtl, responseState, doAbort, doResponse, setAnswerCtl]);

  useEffect(() => {
    if (
      [ResponseStep.Aborted, ResponseStep.Answered, ResponseStep.Error].includes(responseState.step)
    ) {
      doPostResponse();
    }
  }, [responseState, doPostResponse]);

  return {
    response: responseState,
    setAnswerCtl,
  };
};
