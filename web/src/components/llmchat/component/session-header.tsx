import Icon from "@/components/icon";
import LLMChatBotSelect from "./bot-select";
import KButton from "@/container/kbutton";

const Header = ({ onAdd }: { onAdd: () => void }) => {
  return (
    <div className="flex flex-row p-2 border-b border-cborder space-x-2">
      <KButton
        onClick={onAdd}
        children={
          <>
            <Icon.PlusSquare />
            <div>New</div>
          </>
        }
      />
      <LLMChatBotSelect />
    </div>
  );
};

export default Header;
