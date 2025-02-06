import {
  LLMChatBotBodyOpenAIV1,
  LLMChatRecord,
  LLMChatSession,
  useLLMChatStore,
} from "@/store/llmchat";
import { useEffect, useState } from "react";
import { fetchEventSource } from "@microsoft/fetch-event-source";
import { v4 as uuid } from "uuid";
import { toast } from "sonner";
import RecordContent from "./record-content.component";

enum AnswerState {
  Initial,
  Trigger,
  Answering,
  Answered,
  Inserting,
  Done,
}

export const ResponseRecord = ({
  records,
  session,
  appendRecord,
  setAnswering,
  triggerAnswer,
}: {
  records: LLMChatRecord[];
  session: LLMChatSession;
  triggerAnswer: boolean;
  appendRecord: (record: LLMChatRecord) => Promise<void>;
  setAnswering: (flag: boolean) => void;
}) => {
  const { currentBot } = useLLMChatStore();

  const [answer, setAnswer] = useState("");
  const [answerState, setAnswerState] = useState(AnswerState.Initial);
  const [abortSingal, setAbortSignal] = useState<AbortController>();
  const [shouldAbort, setShouldAbort] = useState(false);
  const [responseSessionId, setResponseSessionId] = useState<
    string | undefined
  >();

  useEffect(() => {
    setAnswering(
      !(answerState === AnswerState.Done || answerState === AnswerState.Initial)
    );
  }, [answerState, setAnswering]);

  useEffect(() => {
    if (answerState !== AnswerState.Answered) {
      return;
    }
    setAnswerState(AnswerState.Inserting);

    if (answer === "") {
      console.warn("llm result is empty");
      return;
    }
    const record: LLMChatRecord = {
      id: uuid(),
      session_id: session.id,
      content: answer ?? "",
      role: "assistant",
      pre_record_id: records.at(-1)?.id,
      insert_time: new Date(),
    };

    appendRecord(record).then(() => {
      setAnswer("");
      setAnswering(false);
      setAnswerState(AnswerState.Done);
    });
  }, [answer, answerState, setAnswer, setAnswerState, setAnswering, records]);

  useEffect(() => {
    if (answerState !== AnswerState.Trigger) {
      return;
    }
    setAnswerState(AnswerState.Answering);
    const config = JSON.parse(currentBot!!.body) as LLMChatBotBodyOpenAIV1;

    const body = {
      model: config.model_name,
      messages: records.map((r) => {
        return { role: r.role, content: r.content };
      }),
      stream: true,
    };

    const ctrl = new AbortController();
    setAbortSignal((signal) => {
      if (signal && !signal.signal.aborted) {
        console.info("cancel old llm request");
        signal.abort();
      }

      return ctrl;
    });
    setResponseSessionId(records.at(-1)?.session_id);

    console.log("begin to request:", body);
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
          setAnswerState(AnswerState.Answered);
          return;
        }
        if (text.trim() === "") {
          return;
        }

        const json = JSON.parse(text);
        const choices = json.choices as Array<{
          delta: {
            content: string | null;
          };
        }>;
        let cont = choices.at(0)?.delta.content;
        if (cont && cont.length > 0) {
          setAnswer((old) => {
            return old + cont;
          });
        }
      },
      onclose() {
        console.log("onclose");
        if ((answerState as AnswerState) === AnswerState.Answering) {
          setAnswerState(AnswerState.Answered);
        }
      },
      onerror(err) {
        console.error("onerror, ", err);
        toast.error(`Error when ask for llm result. \n ${err}`);
        setAnswering(false);
        throw err;
      },
    });
  }, [
    answerState,
    records,
    currentBot,
    setAnswerState,
    setAbortSignal,
    setResponseSessionId,
  ]);

  useEffect(() => {
    if (shouldAbort || session.id !== responseSessionId) {
      abortSingal?.abort();
      setAnswerState(AnswerState.Answered);
    }
  }, [shouldAbort, abortSingal, session, responseSessionId, setAnswerState]);

  if (
    records.at(-1)?.role === "user" &&
    (answerState === AnswerState.Done || answerState === AnswerState.Initial) &&
    triggerAnswer
  ) {
    setAnswerState(AnswerState.Trigger);
  }

  return (
    <RecordContent
      content={answer}
      canAbort={answerState === AnswerState.Answering}
      onAbort={() => {
        setShouldAbort(true);
      }}
      canGenerate={true}
      onRegenerate={() => {}}
      role={"answering-llm"}
    />
  );
};
