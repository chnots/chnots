import { useLLMChatStore, LLMChatTemplate } from "@/store/llmchat";
import Icon from "@/common/component/icon";
import clsx from "clsx";
import KSVG from "@/common/component/svg";
import { RefObject, useEffect, useRef, useState } from "react";
import AddTemplate from "@/features/llmchat/component/template-form";

const ContextMenu = ({
  x,
  y,
  onEdit,
  onDelete,
  divRef,
}: {
  x: number;
  y: number;
  onEdit: () => void;
  onDelete: () => void;
  divRef: RefObject<HTMLDivElement | null>;
}) => {
  const items = [
    { title: "Edit", onClick: onEdit },
    { title: "Delete", onClick: onDelete },
  ];

  return (
    <div ref={divRef} className="p-1 m-1">
      <ul
        className="absolute bg-white shadow-md rounded-md border border-gray-200 z-50"
        style={{ left: x, top: y }}
        role="menu"
        aria-label="context-menu"
      >
        {items.map((item) => (
          <li
            key={item.title}
            role="menuitem"
            tabIndex={0}
            className="cursor-pointer hover:bg-gray-200 p-2 list-none"
            onClick={() => {
              console.log("click", item.title);
              item.onClick();
            }}
            onKeyDown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                item.onClick();
              }
            }}
            aria-label={item.title}
          >
            {item.title}
          </li>
        ))}
      </ul>
    </div>
  );
};

const LLMChatTemplateList = ({
  onClickTemplate,
}: {
  onClickTemplate: (template: LLMChatTemplate) => void;
}) => {
  const { listTemplates, insertTemplate, refreshTemplates } = useLLMChatStore();
  const [showNewForm, setShowNewForm] = useState(false);
  const [contextMenuVisable, setContextMenuVisable] = useState<boolean>();
  const selectedTemplate = useRef<LLMChatTemplate>(null);
  const [contextMenuPosition, setContextMenuPosition] = useState({
    x: 0,
    y: 0,
  });

  const items = listTemplates();
  const className =
    " p-2 flex space-x-2 text-black w-auto align-middle justify-center rounded-md";

  const contextMenuRef = useRef<HTMLDivElement>(null);

  const handleContextMenu = (
    e: React.MouseEvent,
    template: LLMChatTemplate
  ) => {
    e.preventDefault();
    setContextMenuPosition({ x: e.clientX, y: e.clientY });
    console.log("add setContextMenuTemplate");
    selectedTemplate.current = template;
    setContextMenuVisable(true);
  };

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (
        contextMenuVisable &&
        contextMenuRef.current &&
        !contextMenuRef.current.contains(e.target as Node)
      ) {
        console.log("remove setContextMenuTemplate");
        contextMenuRef.current = null;
        setContextMenuVisable(false);
      }
    };
    if (!contextMenuVisable) {
      document.removeEventListener("mousedown", handleClickOutside);
    } else {
      document.addEventListener("mousedown", handleClickOutside);
    }
  }, [contextMenuVisable]);

  return (
    <div>
      {showNewForm && selectedTemplate.current && (
        <AddTemplate
          onClose={() => {
            setShowNewForm(false);
          }}
          onSubmit={async (template) => {
            await insertTemplate(template);
            await refreshTemplates();
            return true;
          }}
          template={selectedTemplate.current}
        />
      )}
      <div className="flex flex-row flex-wrap p-3 m-3 text-sm space-x-2 max-w-3xl">
        <button
          className={clsx(className, "bg-blue-50 hover:cursor-pointer")}
          onClick={() => setShowNewForm(true)}
        >
          <Icon.PlusCircle strokeWidth={1.5} />
          <span>Add New Template</span>
        </button>
        {items.map((item: LLMChatTemplate) => (
          <div
            key={item.id}
            onClick={() => onClickTemplate(item)}
            onContextMenu={(event) => {
              handleContextMenu(event, item);
            }}
            className={clsx(className, "hover:cursor-pointer items-center")}
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
      {contextMenuVisable && (
        <ContextMenu
          x={contextMenuPosition.x}
          y={contextMenuPosition.y}
          onEdit={() => {
            setShowNewForm(true);
            setContextMenuVisable(false);
          }}
          onDelete={() => {}}
          divRef={contextMenuRef}
        />
      )}
      <div></div>
    </div>
  );
};

export default LLMChatTemplateList;
