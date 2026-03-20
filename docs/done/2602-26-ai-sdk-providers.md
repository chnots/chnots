# Plan: Multi-Provider LLM Chat System with Proxy Support (Using AI SDK)

## Overview

This document outlines the plan to extend the llm-chat system to support multiple LLM providers using Vercel AI SDK, with unified proxy support through `base_url` configuration for all providers.

## Key Design Decisions

### 1. Unified Proxy Support

- All providers support custom `base_url` to enable proxy servers
- Do not use any environment variables. 
- User can override with explicit configuration in bot settings

### 2. Backward Compatibility

- Existing bots with legacy format continue to work
- Automatic migration from old format to new format
- No database schema changes required

### 3. Configuration Approach

- Support both environment variables and explicit API keys
- Provider selection via dropdown in bot configuration UI
- Preset configurations for common providers (OpenAI, Anthropic, Google, Ollama)

---

## Phased Implementation

### Phase 1: Infrastructure Setup

#### Dependencies to Install

- `ai` - Core AI SDK
- `@ai-sdk/openai` - OpenAI provider
- `@ai-sdk/anthropic` - Anthropic provider
- `@ai-sdk/google` - Google Generative AI provider
- `@ai-sdk/openai-compatible` - OpenAI-compatible provider for local models

#### Files to Create

**`src/krate/llmchat/provider-manager.ts`**

- Core provider factory function
- Creates provider instances based on configuration
- Handles legacy format migration
- Resolves base URLs from config → environment variables → defaults

#### Files to Modify

**`src/krate/llmchat/po.ts`**

- Add new type `LLMChatBotBodyAI` for AI SDK configuration
- Keep `LLMChatBotBodyOpenAIV1` for backward compatibility
- Create union type `LLMChatBotBody` = old format | new format

Type structure:

```typescript
// Legacy format (existing)
type LLMChatBotBodyOpenAIV1 = {
  url: string; // Mapped to base_url
  token: string; // Mapped to api_key
  model_name: string;
};

// New unified format
type LLMChatBotBodyAI = {
  provider: "openai" | "anthropic" | "google" | "openai-compatible";
  api_key?: string; // Optional, falls back to env vars
  base_url: string; // Required for proxy support
  model_name: string;
};
```

---

### Phase 2: Bot Configuration UI Updates

#### Files to Modify

**`src/krate/llmchat/component/bot-form.tsx`**

- Add provider selection dropdown
- Conditional field display based on selected provider
- All providers show `base_url` field (required)
- Add preset configuration buttons for quick setup

**`src/krate/llmchat/component/bot-list.tsx`**

- Display provider badges for each bot
- Show migration indicator for legacy bots
- Add "Migrate" button for legacy format bots

---

### Phase 3: LLM API Call Integration

#### Files to Modify

**`src/krate/llmchat/component/use-llm-response.ts`**

- Import `streamText` from `ai` package
  - handle onError: exit instantly
- Use `createLLMProvider` to get model instance
- Convert bot config to AI SDK message format
- Handle streaming responses with `fullStream`
- Support reasoning/thinking content from different providers
- Standardize error handling across providers

**`src/krate/llmchat/component/record-response.tsx`**

- Update stream processing to handle different chunk types:
  - `text-delta` - Regular text output
  - `reasoning` - Anthropic thinking content
  - `tool-call` - Future tool calling support
  - `error` - Standardized error handling
- Display reasoning content in collapsible UI (existing component)

---

#### Files to Modify

**`src/krate/llmchat/component/use-llm-response.ts`**

- Wrap API calls in try-catch
- Parse errors using centralized handler
- Display error messages in UI

**`src/krate/llmchat/component/record-response.tsx`**

- Add error state display
- Show retry button on errors

---

## Configuration Structure

### Provider Configuration Examples

**OpenAI (Official)**

```json
{
  "provider": "openai",
  "base_url": "https://api.openai.com/v1",
  "model_name": "gpt-5",
  "api_key": "sk-..." 
}
```

**Anthropic (Official)**

```json
{
  "provider": "anthropic",
  "base_url": "https://api.anthropic.com/v1",
  "model_name": "claude-sonnet-4-20250514",
  "api_key": "sk-ant-..."
}
```

**Google (Official)**

```json
{
  "provider": "google",
  "base_url": "https://generativelanguage.googleapis.com/v1beta",
  "model_name": "gemini-2.5-flash",
  "api_key": "AI..."
}
```

**OpenAI-Compatible (Ollama with proxy)**

```json
{
  "provider": "openai-compatible",
  "base_url": "https://your-proxy.com/ollama/v1",
  "model_name": "llama3",
  "api_key": "ollama-key" // Optional
}
```

**OpenAI-Compatible (Local vLLM)**

```json
{
  "provider": "openai-compatible",
  "base_url": "http://localhost:8000/v1",
  "model_name": "meta-llama/Meta-Llama-3.1-8B-Instruct",
  "api_key": "local-key" // Optional
}
```

---

## Success Criteria

### Functional Requirements

- ✅ Support OpenAI, Anthropic, Google, and OpenAI-compatible providers
- ✅ All providers support custom base URLs (proxy)
- ✅ Legacy bots automatically migrate to new format
- ✅ Users can switch between providers seamlessly
- ✅ Streaming responses work for all providers

### Non-Functional Requirements

- ✅ Zero breaking changes to existing functionality
- ✅ Type-safe TypeScript implementation
- ✅ Clear user documentation
- ✅ Performance comparable to current implementation
