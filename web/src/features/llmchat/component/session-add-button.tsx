import Icon from "@/common/component/icon";
import LLMChatBotSelect from "./bot-select";
import KButton from "@/common/component/kbutton";

const AddButton = ({
  onAdd,
  className,
}: {
  onAdd: () => void;
  className?: string;
}) => {
  return (
    <KButton className={className} onClick={onAdd}>
      <Icon.PlusSquare />
      <div>New Session</div>
    </KButton>
  );
};

export default AddButton;
