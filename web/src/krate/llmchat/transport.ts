import { DirectChatTransport, ToolLoopAgent } from "ai";
import type { LLMChatBotBodyAI } from "./po";
import { createLLMProvider, parseBotBody } from "./provider-manager";

export type LLMChatMessageMetadata = {
  usage?: { inputTokens?: number; outputTokens?: number };
};

export function createTransport(botBody: string) {
  const config: LLMChatBotBodyAI = parseBotBody(botBody);
  const model = createLLMProvider(config);

  const agent = new ToolLoopAgent({
    model,
  });

  return new DirectChatTransport({
    agent,
    messageMetadata({ part }) {
      if (part.type === "finish") {
        return {
          usage: {
            inputTokens: part.totalUsage.inputTokens ?? undefined,
            outputTokens: part.totalUsage.outputTokens ?? undefined,
          },
        } satisfies LLMChatMessageMetadata;
      }
    },
  });
}
