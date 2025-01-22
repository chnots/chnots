import { useLLMChatStore, LLMChatTemplate } from "@/store/llmchat";
import Icon from "../icon";
import LLMChatTemplateIcon from "./template-icon.component";
import clsx from "clsx";

function LLMChatTemplateList({
  onClickTemplate,
}: {
  onClickTemplate: (template: LLMChatTemplate) => void;
}) {
  const { listTemplates } = useLLMChatStore();

  const onAdd = () => {};

  const items = listTemplates();
  const className =
    " p-2 flex space-x-2 text-black w-auto align-middle justify-center rounded-md";

  return (
    <div className="flex p-3 m-3 text-sm space-x-2">
      <button className={clsx(className, "bg-blue-50")} onClick={onAdd}>
        <Icon.PlusCircle />
        <span>Add New Template</span>
      </button>
      {items.map((item) => (
        <div
          key={item.id}
          onClick={() => onClickTemplate(item)}
          className={clsx(className, "hover:cursor-pointer")}
        >
          <LLMChatTemplateIcon name={item.icon_name} />
          <span>{item.name}</span>
        </div>
      ))}
    </div>
  );
}

export default LLMChatTemplateList;
