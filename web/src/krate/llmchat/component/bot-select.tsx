import Icon from "@/common/component/icon";
import KSVG from "@/common/component/svg";
import { useRef, useState } from "react";
import { Button as KButton } from "@/common/component/ui/button";
import { LLMChatBot } from "@/krate/llmchat/store/db";
import { useLLMChatStore } from "@/krate/llmchat/store/store";
import * as RadixDropmenu from "@radix-ui/react-dropdown-menu";
import { llmchatBotAdd } from "@/krate/llmchat/store/service";
import BotForm from "./bot-form";

const LLMChatBotSelect = () => {
  const [showBotForm, setShowBotForm] = useState(false);
  const selectedBotRef = useRef<LLMChatBot>(undefined);

  const { bots, currentBot, setCurrentBot, refreshBots } = useLLMChatStore();

  const handleSelect = (id: string) => {
    setCurrentBot(bots.get(id));
  };

  const AddButton = () => {
    return (
      <KButton
        className="py-1 px-2 text-xs w-full space-x-2"
        onClick={() => {
          selectedBotRef.current = undefined;
          setShowBotForm(true);
        }}
      >
        <Icon.PlusCircle className="w-4 h-4" />
        <span className="ml-1">Add Bot</span>
      </KButton>
    );
  };

  const BotComponent = ({
    bot,
    settings,
  }: {
    bot: LLMChatBot;
    settings?: () => void;
  }) => {
    return (
      <KButton className="py-1 px-2 text-xs justify-between w-full">
        <div
          className="flex flex-row space-x-2"
          onClick={() => {
            handleSelect(bot.id);
          }}
        >
          {bot.svg_logo ? (
            <KSVG inner={bot.svg_logo} className="w-4 h-4" />
          ) : (
            <Icon.Bot />
          )}
          <span>{bot.name}</span>
        </div>
        {settings && (
          <Icon.SettingsIcon
            onClick={settings}
            className="size-4 hover:animate-spin"
          />
        )}
      </KButton>
    );
  };

  return (
    <>
      {currentBot ? (
        <RadixDropmenu.Root>
          <RadixDropmenu.Trigger>
            <BotComponent bot={currentBot} />
          </RadixDropmenu.Trigger>
          <RadixDropmenu.Portal>
            <RadixDropmenu.Content className="kc-inactive p-2 rounded-xl space-y-2 shadow-lg border ">
              {[...bots.values()].map((bot) => {
                return (
                  <RadixDropmenu.Item key={bot.id}>
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
        <AddButton />
      )}
      {showBotForm && (
        <BotForm
          onSubmit={async (bot) => {
            await llmchatBotAdd(bot);
            await refreshBots();
            return true;
          }}
          onClose={() => {
            setShowBotForm(false);
          }}
          bot={selectedBotRef.current}
        />
      )}
    </>
  );
};

export default LLMChatBotSelect;
