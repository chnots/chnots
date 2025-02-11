import Icon from "@/common/component/icon";
import LLMChatBotSelect from "./bot-select";
import KButton from "@/common/component/kbutton";

const Header = ({ onAdd }: { onAdd: () => void }) => {
  return (
    <div className="flex flex-row p-2 border-b border-cborder space-x-2">
      <KButton onClick={onAdd}>
        <Icon.PlusSquare />
        <div>New</div>
      </KButton>
      <LLMChatBotSelect />
    </div>
  );
};

export default Header;
