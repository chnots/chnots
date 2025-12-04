import * as RadixDropmenu from "@radix-ui/react-dropdown-menu";
import { useCallback, useEffect, useRef, useState } from "react";
import Icon from "@/common/component/icon";
import KSVG from "@/common/component/svg";
import type { LLMChatBot } from "@/krate/llmchat/po";
import { llmchatBotCommit } from "@/krate/llmchat/service";
import { useLLMChatStore } from "@/krate/llmchat/store";
import type { TID } from "@/lib/id_util";
import BotForm from "./bot-form";
import { useLLMChatComStore } from "./session";

const BotComponent = ({
  bot,
  settings,
}: {
  bot: LLMChatBot;
  settings?: () => void;
}) => {
  const { bots } = useLLMChatStore();
  const { setBot } = useLLMChatComStore((store) => {
    return {
      setBot: store.setBot,
    };
  });


  const handleSelect = (tid: TID) => {
    const bot = bots.get(tid);
    if (bot) {
      setBot(bot);
    }
  };


  return (
    <div
      role="none"
      className="w-full flex justify-between text-xs border py-1 px-2 items-center rounded-md hover:cursor-pointer"
      onClick={() => {
        handleSelect(bot.otid);
      }}
    >
      <div className="flex flex-row space-x-2 items-center">
        {bot.svg_logo ? (
          <KSVG src={bot.svg_logo} className="w-4 h-4" />
        ) : (
          <Icon.Bot className="w-4 h-4" />
        )}
        <span>{bot.name}</span>
      </div>
      {settings && (
        <Icon.SettingsIcon
          onClick={settings}
          className="size-4 hover:animate-spin"
        />
      )}
    </div>
  );
};


const LLMChatBotSelect = () => {
  const [showBotForm, setShowBotForm] = useState(false);
  const selectedBotRef = useRef<LLMChatBot>(undefined);

  const { bots, refreshBots } = useLLMChatStore();
  const { bot, setBot } = useLLMChatComStore((store) => {
    return {
      bot: store.bot,
      setBot: store.setBot,
    };
  });


  useEffect(() => {
    if (!bot) {
      if (bots.size > 0) {
        setBot([...bots.values()][0]);
      }
    }
  }, [bots, bot, setBot]);

  const AddButton = () => {
    return (
      <div
        role="none"
        className="w-full flex justify-between text-xs border py-1 px-2 items-center rounded-md cursor-pointer"
        onClick={() => {
          selectedBotRef.current = undefined;
          setShowBotForm(true);
        }}
      >
        <Icon.PlusCircle className="w-4 h-4" />
        <span className="ml-1">Add Bot</span>
      </div>
    );
  };


  const handleSubmit = useCallback(
    async (bot: LLMChatBot) => {
      await llmchatBotCommit(bot);
      await refreshBots();
      return true;
    },
    [refreshBots],
  );

  const handleClose = useCallback(() => {
    setShowBotForm(false);
  }, []);
  return (
    <>
      {bot ? (
        <RadixDropmenu.Root>
          <RadixDropmenu.Trigger>
            <BotComponent bot={bot} />
          </RadixDropmenu.Trigger>
          <RadixDropmenu.Portal>
            <RadixDropmenu.Content className="kc-inactive p-2 rounded-xl space-y-2 shadow-lg border z-51">
              {[...bots.values()].map((bot) => {
                return (
                  <RadixDropmenu.Item key={bot.otid}>
                    <BotComponent
                      bot={bot}
                      settings={() => {
                        selectedBotRef.current = bot;
                        setShowBotForm(true);
                      }}
                    />
                  </RadixDropmenu.Item>
                );
              })}
              <RadixDropmenu.Item key="Add">
                <AddButton />
              </RadixDropmenu.Item>
            </RadixDropmenu.Content>
          </RadixDropmenu.Portal>
        </RadixDropmenu.Root>
      ) : (
        <div>
          <AddButton />
        </div>
      )}
      {showBotForm && (
        <BotForm
          onSubmit={handleSubmit}
          onClose={handleClose}
          bot={selectedBotRef.current}
        />
      )}
    </>
  );
};

export default LLMChatBotSelect;
