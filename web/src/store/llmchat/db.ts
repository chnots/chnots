// Date type definition for compatibility
type DateTime = Date;

// LLMChatBot structure
export interface LLMChatBot {
  id: string;
  name: string;
  body: string;
  svg_logo?: string;
  delete_time?: DateTime; // Optional field
  update_time?: DateTime; // Optional field
  insert_time: DateTime;
}

// the body of LLMChatBot body.
export interface LLMChatBotBodyOpenAIV1 {
  url: string;
  token: string;
  model_name: string;
}

// LLMChatTemplate structure
export interface LLMChatTemplate {
  id: string;
  name: string;
  prompt: string;
  svg_logo?: string;
  delete_time?: DateTime; // Optional field
  update_time?: DateTime; // Optional field
  insert_time: DateTime;
}

// LLMChatSession structure
export interface LLMChatSession {
  id: string;
  bot_id: string;
  template_id: string;
  title: string;
  namespace: string;
  delete_time?: DateTime; // Optional field
  update_time?: DateTime; // Optional field
  insert_time: DateTime;
}

// LLMChatRecord structure
export interface LLMChatRecord {
  id: string;
  session_id: string;
  pre_record_id?: string; // Optional field
  content: string;
  role: string;
  role_id?: string;
  insert_time: DateTime;
}