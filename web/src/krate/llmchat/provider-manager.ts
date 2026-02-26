import { createAnthropic } from "@ai-sdk/anthropic";
import { createGoogleGenerativeAI } from "@ai-sdk/google";
import { createOpenAI } from "@ai-sdk/openai";
import { createOpenAICompatible } from "@ai-sdk/openai-compatible";
import type { LanguageModel } from "ai";
import type {
  LLMChatBotBodyAI,
  LLMChatBotBodyOpenAIV1,
  LLMProvider,
} from "./po";

const DEFAULT_BASE_URLS: Record<LLMProvider, string> = {
  openai: "https://api.openai.com/v1",
  anthropic: "https://api.anthropic.com/v1",
  google: "https://generativelanguage.googleapis.com/v1beta",
  "openai-compatible": "",
};

const isLegacyFormat = (body: unknown): body is LLMChatBotBodyOpenAIV1 => {
  return (
    typeof body === "object" &&
    body !== null &&
    "url" in body &&
    "token" in body &&
    "model_name" in body &&
    !("provider" in body)
  );
};

export const migrateToAIFormat = (
  legacy: LLMChatBotBodyOpenAIV1,
): LLMChatBotBodyAI => {
  return {
    provider: "openai-compatible",
    base_url: legacy.url,
    api_key: legacy.token,
    model_name: legacy.model_name,
  };
};

export const parseBotBody = (bodyStr: string): LLMChatBotBodyAI => {
  const parsed = JSON.parse(bodyStr) as
    | LLMChatBotBodyAI
    | LLMChatBotBodyOpenAIV1;

  if (isLegacyFormat(parsed)) {
    return migrateToAIFormat(parsed);
  }

  return parsed;
};

export const createLLMProvider = (config: LLMChatBotBodyAI): LanguageModel => {
  const { provider, api_key, base_url, model_name } = config;

  switch (provider) {
    case "openai": {
      const openai = createOpenAI({
        apiKey: api_key,
        baseURL: base_url || DEFAULT_BASE_URLS.openai,
      });
      return openai(model_name);
    }

    case "anthropic": {
      const anthropic = createAnthropic({
        apiKey: api_key,
        baseURL: base_url || DEFAULT_BASE_URLS.anthropic,
      });
      return anthropic(model_name);
    }

    case "google": {
      const google = createGoogleGenerativeAI({
        apiKey: api_key,
        baseURL: base_url || DEFAULT_BASE_URLS.google,
      });
      return google(model_name);
    }

    case "openai-compatible": {
      const compatible = createOpenAICompatible({
        apiKey: api_key || "local",
        baseURL: base_url,
        name: "openai-compatible",
      });
      return compatible(model_name);
    }

    default:
      throw new Error(`Unknown provider: ${provider}`);
  }
};

export const getProviderDefaultBaseUrl = (provider: LLMProvider): string => {
  return DEFAULT_BASE_URLS[provider];
};

export const PROVIDER_OPTIONS: { value: LLMProvider; label: string }[] = [
  { value: "openai", label: "OpenAI" },
  { value: "anthropic", label: "Anthropic" },
  { value: "google", label: "Google" },
  { value: "openai-compatible", label: "OpenAI Compatible" },
];

export const PRESET_CONFIGS: Record<string, Partial<LLMChatBotBodyAI>> = {
  openai: {
    provider: "openai",
    base_url: "https://api.openai.com/v1",
    model_name: "gpt-4o",
  },
  anthropic: {
    provider: "anthropic",
    base_url: "https://api.anthropic.com/v1",
    model_name: "claude-sonnet-4-20250514",
  },
  google: {
    provider: "google",
    base_url: "https://generativelanguage.googleapis.com/v1beta",
    model_name: "gemini-2.5-flash",
  },
  ollama: {
    provider: "openai-compatible",
    base_url: "http://localhost:11434/v1",
    model_name: "llama3",
    api_key: "ollama",
  },
};
