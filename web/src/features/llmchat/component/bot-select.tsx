import Icon from "@/common/component/icon";
import KSVG from "@/common/component/svg";
import { LLMChatBot, useLLMChatStore } from "@/store/llmchat";
import React, { useRef, useState } from "react";
import BotForm from "./bot-form";
import KButton from "@/common/component/kbutton";

const BotComponent = ({
  bot,
  settings,
}: {
  bot: LLMChatBot;
  settings?: () => void;
}) => {
  return (
    <KButton className="py-1 px-2">
      <div className="flex flex-row items-center space-x-2">
        {bot.svg_logo ? (
          <KSVG inner={bot.svg_logo} className="w-4 h-4" />
        ) : (
          <Icon.Bot />
        )}
        <span>{bot.name}</span>
        {settings && (
          <Icon.SettingsIcon
            onClick={settings}
            className="size-4 hover:animate-spin justify-self-end"
          />
        )}
      </div>
    </KButton>
  );
};

const LLMChatBotSelect = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [showBotForm, setShowBotForm] = useState(false);
  const selectedBotRef = useRef<LLMChatBot>(undefined);

  const handleToggle = () => {
    setIsOpen(!isOpen);
  };

  const { bots, currentBot, setCurrentBot, insertBot, refreshBots } =
    useLLMChatStore();

  const handleSelect = (id: string) => {
    setCurrentBot(bots.get(id));
    setIsOpen(false);
  };

  return (
    <div>
      {currentBot ? (
        <div onClick={handleToggle}>
          <BotComponent bot={currentBot} />
        </div>
      ) : (
        <div>None bots?</div>
      )}
      {isOpen && (
        <ul className="absolute mt-1 py-1 bottom-16 rounded-md shadow-lg z-10 border border-gray-300 bg-white list-none">
          {[...bots.values()].map((bot) => (
            <li
              key={bot.id}
              className="px-1 py-1 hover:bg-gray-100 cursor-pointer bg-white"
              onClick={() => handleSelect(bot.id)}
            >
              <BotComponent
                bot={bot}
                settings={() => {
                  selectedBotRef.current = bot;
                  setShowBotForm(true);
                }}
              />
            </li>
          ))}
          <li
            className="px-1 py-1 hover:bg-gray-100 cursor-pointer bg-white"
            onClick={() => {
              selectedBotRef.current = undefined;
              setShowBotForm(true);
              setIsOpen(false);
            }}
          >
            <KButton className="py-1 px-2">
              <Icon.PlusCircle />
              <span className="ml-1">Add Bot</span>
            </KButton>
          </li>
        </ul>
      )}
      {showBotForm && (
        <BotForm
          onSubmit={async (bot) => {
            await insertBot(bot);
            await refreshBots();
            return true;
          }}
          onClose={() => {
            setShowBotForm(false);
          }}
          bot={selectedBotRef.current}
        />
      )}
    </div>
  );
};

export default LLMChatBotSelect;
