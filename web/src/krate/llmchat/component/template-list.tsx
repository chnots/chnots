import Icon from "@/common/component/icon";
import KSVG from "@/common/component/svg";
import { Button } from "@/common/component/ui/button";
import type { LLMChatTemplate } from "@/krate/llmchat/po";
import { useLLMChatStore } from "@/krate/llmchat/store";

const LLMChatTemplateList = ({
  onSelectTemplate,
  onNew,
}: {
  onSelectTemplate: (template: LLMChatTemplate) => void;
  onNew: () => void;
}) => {
  const { templates } = useLLMChatStore();

  return (
    <div className="flex flex-wrap space-x-4 text-sm p-3">
      {[...templates.values()].map((item) => (
        <Button
          key={item.otid}
          className={"hover:cursor-pointer space-x-1 items-center flex py-1"}
          onClick={() => {
            onSelectTemplate(item);
          }}
        >
          {item.svg_logo ? (
            <KSVG src={item.svg_logo} className="w-4 h-4" />
          ) : (
            <Icon.MessageCircle className="w-4 h-4" />
          )}
          <span>{item.name}</span>
        </Button>
      ))}
      <Button
        key={"add-new"}
        className={"hover:cursor-pointer space-x-1 items-center flex py-1"}
        onClick={() => {
          onNew();
        }}
      >
        <Icon.BadgePlus />
      </Button>
    </div>
  );
};

export default LLMChatTemplateList;
