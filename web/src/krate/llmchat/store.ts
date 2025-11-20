import { create } from 'zustand';
import { combine } from 'zustand/middleware';

import { llmchatBotList, llmchatTemplateList } from './service';

import type { LLMChatBotListRsp } from './dto';
import type { LLMChatBot, LLMChatTemplate } from './po';
import type { TID } from '@/lib/id_util';

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
      const bots: LLMChatBotListRsp = await llmchatBotList();
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
