import Icon from "@/common/component/icon";
import clsx from "clsx";
import KSVG from "@/common/component/svg";
import { useRef } from "react";
import { LLMChatTemplate } from "@/krate/llmchat/po";
import { useLLMChatStore } from "@/krate/llmchat/store";
import { Button } from "@/common/component/ui/button";
import { useIsMobile } from "@/hooks/use-mobile";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/common/component/ui/dropdown-menu";
import { DialogTrigger } from "@/common/component/ui/dialog";
import { genTID } from "@/lib/id_util";

const LLMChatTemplateList = ({
  onClickTemplate,
  onChangeEditTemplate,
}: {
  onClickTemplate: (template: LLMChatTemplate) => void;
  onChangeEditTemplate: (template: LLMChatTemplate) => void;
}) => {
  const isMobile = useIsMobile();
  const { listTemplates, refreshTemplates } = useLLMChatStore();
  const selectedTemplate = useRef<LLMChatTemplate>(undefined);

  const items = listTemplates();
  const className =
    " p-2 flex space-x-2 text-black w-auto align-mie justify-center rounded-md";

  return (
    <>
      <div className="flex flex-row flex-wrap p-3 m-3 text-sm space-x-2 max-w-3xl align-middle items-center">
        <DialogTrigger>
          <Button
            className={clsx(className, "bg-blue-50 hover:cursor-pointer")}
            onClick={(e) =>
              onChangeEditTemplate({
                tid: genTID(),
                name: "",
                prompt: "",
              })
            }
            asChild
          >
            <div>
              <Icon.PlusCircle strokeWidth={1.5} />
              <span>Add New Template</span>
            </div>
          </Button>
        </DialogTrigger>
        {items.map((item: LLMChatTemplate) => (
          <div
            key={item.tid}
            className={clsx(className, "hover:cursor-pointer items-center")}
          >
            <DropdownMenu>
              <DropdownMenuTrigger>
                {item.svg_logo ? (
                  <KSVG inner={item.svg_logo} />
                ) : (
                  <Icon.MessageCircle />
                )}
              </DropdownMenuTrigger>
              <DropdownMenuContent
                className="w-48 rounded-lg"
                side={isMobile ? "bottom" : "right"}
                align={isMobile ? "end" : "start"}
              >
                <DialogTrigger>
                  <DropdownMenuItem
                    onSelect={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      onChangeEditTemplate(item);
                    }}
                  >
                    Edit
                  </DropdownMenuItem>
                </DialogTrigger>
                <DropdownMenuItem onClick={(e) => e.preventDefault()}>
                  Delete
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>

            <span onClick={() => onClickTemplate(item)}>{item.name}</span>
          </div>
        ))}
      </div>
    </>
  );
};

export default LLMChatTemplateList;
