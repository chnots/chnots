import { useLLMChatStore, LLMChatTemplate } from "@/store/llmchat";
import Icon from "../../icon";
import clsx from "clsx";
import KSVG from "../../svg";

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
    <div>
      <div className="flex flex-row p-3 m-3 text-sm space-x-2">
        <button className={clsx(className, "bg-blue-50")} onClick={onAdd}>
          <Icon.PlusCircle />
          <span>Add New Template</span>
        </button>
        {items.map((item: LLMChatTemplate) => (
          <div
            key={item.id}
            onClick={() => onClickTemplate(item)}
            className={clsx(className, "hover:cursor-pointer")}
          >
            {item.svg_logo ? (
              <KSVG inner={item.svg_logo} />
            ) : (
              <Icon.MessageCircle />
            )}
            <span>{item.name}</span>
          </div>
        ))}
      </div>
      <div></div>
    </div>
  );
}

export default LLMChatTemplateList;
