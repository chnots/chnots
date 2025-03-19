import { insertMapAtIndex } from "@/utils/map-utils";
import request from "@/utils/request";
import { create } from "zustand";
import { combine } from "zustand/middleware";
import {
  LLMChatSession,
  LLMChatBot,
  LLMChatTemplate,
  LLMChatRecord,
} from "./db";
import {
  LLMChatListBotRsp,
  LLMChatListTemplateRsp,
  LLMChatListSessionRsp,
  LLMChatSessionDetailRsp,
  LLMChatSessionUpdateReq,
  LLMChatSessionTruncateReq,
} from "./dto";
import {
  llmchatBotAdd,
  llmchatBotList,
  llmchatSessionList,
  llmchatSessionOverwrite,
  llmchatTemplateList,
} from "./service";

interface State {
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
    bots: new Map(),
    templates: new Map(),
    sessions: new Map(),
  };
};

export const useLLMChatStore = create(
  combine(getDefaultState(), (set, get) => ({
    getState: () => get(),
    refreshBots: async () => {
      const bots: LLMChatListBotRsp = await llmchatBotList();
      set((state) => {
        return {
          ...state,
          bots: new Map(bots.bots.map((e) => [e.id, e])),
          currentBot: bots.bots.at(0),
        };
      });
    },
    refreshTemplates: async () => {
      const templates = await llmchatTemplateList();
      set((state) => {
        return {
          ...state,
          templates: new Map(templates.templates.map((e) => [e.id, e])),
        };
      });
    },
    refreshSessions: async () => {
      const sessions: LLMChatListSessionRsp = await llmchatSessionList();
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
    unshiftSession: async (session: LLMChatSession) => {
      await llmchatSessionOverwrite(session);
      const sessions = get().sessions;
      set((state) => {
        return {
          ...state,
          sessions: insertMapAtIndex(0, session.id, session, sessions),
        };
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
