import { insertMapAtIndex } from "@/lib/map-utils";
import { create } from "zustand";
import { combine } from "zustand/middleware";
import { LLMChatSession, LLMChatBot, LLMChatTemplate } from "./po";
import { LLMChatListBotRsp, LLMChatListSessionRsp } from "./dto";
import {
  llmchatBotList,
  llmchatSessionList,
  llmchatSessionOverwrite,
  llmchatTemplateList,
} from "./service";
import { TID } from "@/lib/id_util";

interface State {
  refreshSessions: () => void;
  refreshTemplates: () => void;
  refreshBots: () => void;
  bots: Map<TID, LLMChatBot>;
  templates: Map<TID, LLMChatTemplate>;
  sessions: Map<TID, LLMChatSession>;
  currentSessionId?: TID;
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
          bots: new Map(bots.bots.map((e) => [e.otid, e])),
          currentBot: bots.bots.at(0),
        };
      });
    },
    refreshTemplates: async () => {
      const templates = await llmchatTemplateList();
      set((state) => {
        return {
          ...state,
          templates: new Map(templates.templates.map((e) => [e.otid, e])),
        };
      });
    },
    refreshSessions: async () => {
      const sessions: LLMChatListSessionRsp = await llmchatSessionList();
      set((state) => {
        return {
          ...state,
          sessions: new Map(sessions.sessions.map((e) => [e.otid, e])),
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
      await llmchatSessionOverwrite({
        ...session,
        title: session.title.substring(0, 200),
      });
      const sessions = get().sessions;
      set((state) => {
        return {
          ...state,
          sessions: insertMapAtIndex(0, session.otid, session, sessions),
        };
      });
    },

    deleteCacheSession: async (sessionId: TID) => {
      set((state) => {
        const sessions = state.sessions;
        sessions.delete(sessionId);
        const currentSessionId =
          state.currentSessionId === sessionId
            ? undefined
            : state.currentSessionId;
        return {
          ...state,
          sessions,
          currentSessionId,
        };
      });
    },

    setCurrentSessionId: (sessionId?: TID) => {
      set((state) => {
        return { ...state, currentSessionId: sessionId };
      });
    },
    setCurrentBot: (bot?: LLMChatBot) => {
      set((state) => {
        return { ...state, currentBot: bot };
      });
    },
  }))
);
