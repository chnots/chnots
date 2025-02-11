import Icon from "@/components/icon";
import KSVG from "@/components/svg";
import { LLMChatBot, useLLMChatStore } from "@/store/llmchat";
import React, { useRef, useState } from "react";
import BotForm from "./bot-form";
import KButton from "@/container/kbutton";

const BotComponent = ({
  bot,
  settings,
}: {
  bot: LLMChatBot;
  settings?: () => void;
}) => {
  return (
    <KButton>
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

const LLMChatBotSelect: React.FC<{}> = ({}) => {
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
    <div className="">
      {currentBot ? (
        <div onClick={handleToggle}>
          <BotComponent bot={currentBot} />
        </div>
      ) : (
        <div>None bots?</div>
      )}
      {isOpen && (
        <ul className="absolute mt-2 py-2 rounded-md shadow-lg z-10 border border-gray-300 bg-white list-none">
          {[...bots.values()].map((bot) => (
            <li
              key={bot.id}
              className="px-4 py-2 hover:bg-gray-100 cursor-pointer bg-white"
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
            className="px-4 py-2 hover:bg-gray-100 cursor-pointer bg-white"
            onClick={() => {
              selectedBotRef.current = undefined;
              setShowBotForm(true);
              setIsOpen(false);
            }}
          >
            <div className="flex items-center cursor-pointer p-2 rounded hover:border-gray-400">
              <Icon.PlusCircle />
              <span className="ml-2">Add Bot</span>
            </div>
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
