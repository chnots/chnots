import Icon from "@/components/icon";
import LLMChatBotSelect from "./bot-select";
import AddButton from "@/components/add-button";

const Header = ({ onAdd }: { onAdd: () => void }) => {
  return (
    <div className="flex flex-row p-2 border-b space-x-2">
      <AddButton onAdd={onAdd} />
      <LLMChatBotSelect />
    </div>
  );
};

export default Header;
