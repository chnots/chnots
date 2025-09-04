import { create } from "zustand";
import { combine } from "zustand/middleware";
import { LLMChatSession, LLMChatBot, LLMChatTemplate } from "./po";
import { LLMChatListBotRsp } from "./dto";
import { llmchatBotList, llmchatTemplateList } from "./service";
import { TID } from "@/lib/id_util";

interface State {
  refreshTemplates: () => void;
  refreshBots: () => void;
  bots: Map<TID, LLMChatBot>;
  templates: Map<TID, LLMChatTemplate>;
}

const getDefaultState = (): State => {
  return {
    refreshTemplates: () => {},
    refreshBots: () => {},
    bots: new Map(),
    templates: new Map(),
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
  })),
);
