import { LLMChatBot, useLLMChatStore } from "@/store/llmchat";
import React, { useState } from "react";
import Icon from "../icon";
import KSVG from "../svg";

const BotComponent = ({ bot }: { bot: LLMChatBot }) => (
  <div className="flex items-center cursor-pointer p-2 border border-gray-300 rounded hover:border-gray-400">
    {bot.svg_logo ? (
      <KSVG inner={bot.svg_logo} className="w-8 h-8" />
    ) : (
      <Icon.Bot />
    )}
    <span className="ml-2">{bot.name}</span>
  </div>
);

const LLMChatBotSelect: React.FC<{}> = ({}) => {
  const [isOpen, setIsOpen] = useState(false);

  const handleToggle = () => {
    setIsOpen(!isOpen);
  };

  const { bots, currentBot, setCurrentBot } = useLLMChatStore();

  const handleSelect = (id: string) => {
    setCurrentBot(bots.get(id));
    setIsOpen(false);
  };

  return (
    <div className="h-16">
      {currentBot ? (
        <div onClick={handleToggle}>
          <BotComponent bot={currentBot} />
        </div>
      ) : (
        <div>None bots?</div>
      )}
      {isOpen && (
        <div className="absolute mt-2 py-2 rounded-md shadow-lg z-10 border border-gray-300">
          {[...bots.values()].map((bot) => (
            <div
              key={bot.id}
              className="px-4 py-2 hover:bg-gray-100 cursor-pointer bg-white"
              onClick={() => handleSelect(bot.id)}
            >
              <BotComponent bot={bot} />
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default LLMChatBotSelect;
