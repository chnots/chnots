import { DirectChatTransport, ToolLoopAgent } from "ai";
import type { LLMChatBotBodyAI } from "./po";
import { createLLMProvider, parseBotBody } from "./provider-manager";

export function createTransport(botBody: string) {
  const config: LLMChatBotBodyAI = parseBotBody(botBody);
  const model = createLLMProvider(config);

  const agent = new ToolLoopAgent({
    model,
  });

  return new DirectChatTransport({ agent });
}
