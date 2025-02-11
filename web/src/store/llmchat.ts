import { insertMapAtIndex } from "@/utils/map-utils";
import request from "@/utils/request";
import { create } from "zustand";
import { combine } from "zustand/middleware";

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

export interface LLMChatListBotRsp {
  bots: LLMChatBot[];
}

export interface LLMChatListTemplateRsp {
  templates: LLMChatTemplate[];
}

export interface LLMChatListSessionRsp {
  sessions: LLMChatSession[];
}

export interface LLMChatSessionDetailRsp {
  records: LLMChatRecord[];
}

export interface LLMChatSessionUpdateReq {
  session_id: string;
  delete?: boolean;
  title?: string;
}

export interface LLMChatSessionDetail {
  session: LLMChatSession;
  records: LLMChatRecord[];
  persisted: boolean;
}

interface State {
  insertSession: (session: LLMChatSession) => void;
  refreshSessions: () => void;
  refreshTemplates: () => void;
  refreshBots: () => void;
  bots: Map<string, LLMChatBot>;
  templates: Map<string, LLMChatTemplate>;
  sessions: Map<string, LLMChatSession>;
  currentSession?: LLMChatSession;
  currentBot?: LLMChatBot;
}

const getDefaultState = (): State => {
  return {
    refreshSessions: () => {},
    refreshTemplates: () => {},
    refreshBots: () => {},
    insertSession: (session: LLMChatSession) => {},
    bots: new Map(),
    templates: new Map(),
    sessions: new Map(),
  };
};

export const useLLMChatStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
    refreshBots: async () => {
      const bots: LLMChatListBotRsp = await request.get(`api/v1/llmchat/bots`);
      set((state) => {
        return {
          ...state,
          bots: new Map(bots.bots.map((e) => [e.id, e])),
          currentBot: bots.bots.at(0),
        };
      });
    },
    refreshTemplates: async () => {
      const templates: LLMChatListTemplateRsp = await request.get(
        `api/v1/llmchat/templates`
      );
      set((state) => {
        return {
          ...state,
          templates: new Map(templates.templates.map((e) => [e.id, e])),
        };
      });
    },
    refreshSessions: async () => {
      const sessions: LLMChatListSessionRsp = await request.get(
        `api/v1/llmchat/sessions`
      );
      set((state) => {
        return {
          ...state,
          sessions: new Map(sessions.sessions.map((e) => [e.id, e])),
        };
      });
    },
    refreshAll: async () => {
      get().refreshSessions();
      get().refreshTemplates();
      get().refreshBots();
    },

    listBots: () => {
      return [...get().bots.values()];
    },
    listTemplates: () => {
      return [...get().templates.values()];
    },
    insertTemplate: async (template: LLMChatTemplate) => {
      return await request.put<object>(`api/v1/llmchat/template`, {
        template,
      });
    },
    insertBot: async (bot: LLMChatBot) => {
      return await request.put<object>(`api/v1/llmchat/bot`, {
        bot,
      });
    },
    fetchSessionRecords: async (session: LLMChatSession) => {
      return await request.get<LLMChatSessionDetailRsp>(
        `api/v1/llmchat/session`,
        {
          params: {
            session_id: session.id,
          },
        }
      );
    },
    insertSession: async (session: LLMChatSession) => {
      await request.put(`api/v1/llmchat/session`, {
        session: session,
      });
    },
    unshiftSession: async (session: LLMChatSession) => {
      await get().insertSession(session);
      const sessions = get().sessions;
      set((state) => {
        return {
          ...state,
          sessions: insertMapAtIndex(0, session.id, session, sessions),
        };
      });
    },
    updateSession: async (req: LLMChatSessionUpdateReq) => {
      await request.post(`api/v1/llmchat/session`, {
        ...req,
      });
    },
    deleteCacheSession: async (sessionId: string) => {
      set((state) => {
        const sessions = state.sessions;
        sessions.delete(sessionId);
        const currentSession =
          state.currentSession?.id === sessionId
            ? undefined
            : state.currentSession;
        return {
          ...state,
          sessions,
          currentSession,
        };
      });
    },
    insertRecord: async (record: LLMChatRecord) => {
      await request.put(`api/v1/llmchat/record`, {
        record: record,
      });
    },
    setCurrentSession: (session?: LLMChatSession) => {
      set((state) => {
        return { ...state, currentSession: session };
      });
    },
    setCurrentBot: (bot?: LLMChatBot) => {
      set((state) => {
        return { ...state, currentBot: bot };
      });
    },
  }))
);
