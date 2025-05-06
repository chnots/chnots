import React, {
  ButtonHTMLAttributes,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import Icon from "@/common/component/icon";
import useDebounce from "@/hooks/use-debounce";
import { useNamespaceStore } from "@/store/namespace";
import { NamespaceSelect } from "@/common/component/namespace-select";
import clsx, { ClassValue } from "clsx";
import useResizeObserver from "@react-hook/resize-observer";
import {
  Chnot,
  ChnotOverwriteReq,
  list_view_type_get_tag_path,
} from "@/store/chnot/dto";
import { useChnotStore } from "@/store/chnot/store";
import { chnotUpdate } from "@/store/chnot/service";
import MarkdownViewer from "./chnot-markdown-viewer";
import MarkdownEditor from "./chnot-markdown-editor";
import { ChnotType } from "@/store/chnot/db";
import ExcalidrawContainer from "@/features/tool/excalidraw/component/excalidraw-container";
import KButton, { KButtonProps } from "@/common/component/kbutton";
import { enumFromStringValue } from "@/utils/enum-util";

import * as RadixPopover from "@radix-ui/react-popover";

enum RequestState {
  Saved,
  Requesting,
  Error,
}

interface ChnotEditState {
  isUploadingResource: boolean;
  requestState: RequestState;
  isComposing: boolean;
}

const TopbarButton = (props: KButtonProps) => {
  return (
    <KButton {...props} className={clsx("px-2 py-1", props.className)}>
      {props.children}
    </KButton>
  );
};

export const ChnotContainer = ({
  chnot,
  className,
  globalViewMode,
  onChnotChange,
  onClickNewButton,
}: {
  className?: string;
  chnot?: Chnot;
  onChnotChange?: (chnot: Chnot) => void;
  onClickNewButton?: () => void;
  globalViewMode?: React.RefObject<boolean>;
}) => {
  const { currentNamespace } = useNamespaceStore();
  const { overwriteChnot, validateChnotCache, listViewType } = useChnotStore();

  const [editState, setEditState] = useState<ChnotEditState>({
    isUploadingResource: false,
    requestState: RequestState.Saved,
    isComposing: false,
  });

  const [viewMode, setViewMode] = useState(globalViewMode?.current ?? false);

  const cmRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(undefined);
  useResizeObserver<HTMLDivElement>(cmRef, (entry) => {
    setHeight(entry.contentRect.height);
  });

  const [chnotType, setChnotType] = useState<ChnotType>(() => {
    const ct = enumFromStringValue(ChnotType, chnot?.meta.kind);
    return ct ? ct : ChnotType.MarkdownWithToent;
  });

  const saveContent = useCallback(
    async (content?: string) => {
      if (content === null || content === undefined) {
        return;
      }
      setEditState((state) => {
        return {
          ...state,
          requestState: RequestState.Requesting,
        };
      });
      let requestState;

      try {
        const req: ChnotOverwriteReq = {
          content: content ?? "",
          insert_time: new Date(),
          meta_id: chnot?.meta.id,
          kind: chnotType,
        };

        const rsp = await overwriteChnot(req, true);
        if (onChnotChange) {
          onChnotChange(rsp.chnot);
        }
        requestState = RequestState.Saved;
      } catch {
        requestState = RequestState.Error;
      }

      setEditState((state) => {
        return {
          ...state,
          requestState,
        };
      });
    },
    [setEditState, editState, onChnotChange, chnot, chnotType]
  );

  useEffect(() => {
    const init = async () => {
      if (chnotType == ChnotType.ExcalidrawV1) {
        if (!chnot) {
          saveContent(
            `# Excalidraw -- ${new Date().toLocaleTimeString()}

${list_view_type_get_tag_path(listViewType) ?? ""}
`
          );
        }
      }
    };
    init();
  }, [chnot, chnotType]);

  const onChange = useDebounce(
    (content: string) => {
      saveContent(content);
    },
    1000,
    true
  );

  const ChnotTypeButton = ({
    thisChnotType,
    children,
  }: {
    thisChnotType: ChnotType;
    children: React.ReactNode;
  }) => {
    return (
      <TopbarButton
        className={chnotType === thisChnotType ? "kc-accent" : ""}
        onClick={() => {
          setChnotType(thisChnotType);
        }}
      >
        {children}
      </TopbarButton>
    );
  };

  return (
    <div className={clsx(className, "flex flex-col h-full")}>
      <div className="w-full flex items-center border-b kc-basic-with-bdr px-3 justify-between text-xs align-middle">
        <div className="text-xs flex space-x-2 p-1 items-center">
          {onClickNewButton &&
            (chnot ? (
              <TopbarButton
                className="py-1.5"
                onClick={() => onClickNewButton()}
              >
                <Icon.BadgePlus className="w-4 h-4" />
                <span>New</span>
              </TopbarButton>
            ) : (
              <div className="flex border kc-active rounded-xl p-0.5 space-x-2">
                <ChnotTypeButton thisChnotType={ChnotType.MarkdownWithToent}>
                  <Icon.TextCursor className="w-4 h-4" />
                </ChnotTypeButton>
                <ChnotTypeButton thisChnotType={ChnotType.ExcalidrawV1}>
                  <Icon.PencilRuler className="w-4 h-4" />
                </ChnotTypeButton>
              </div>
            ))}
          {chnot && (
            <div className="bg-inactive border kc-active rounded-xl p-0.5 flex space-x-1">
              <NamespaceSelect
                className="w-4 h-4"
                menuClassName="px-1 py-1 bg-inactive rounded-xl flex items-center "
                onSelect={(ns) => {
                  chnotUpdate({
                    meta_id: chnot.meta.id,
                    update_time: false,
                    namespace: ns,
                  }).then((_) => {
                    if (ns !== currentNamespace.name) {
                      validateChnotCache([chnot.meta.id]);
                    }
                  });
                }}
                currentNamespace={chnot.meta.namespace}
              />
              <TopbarButton
                onClick={() => {
                  if (globalViewMode) {
                    globalViewMode.current = !viewMode;
                  }
                  setViewMode((prev) => !prev);
                }}
              >
                <Icon.Eye className="w-4 h-4" />
              </TopbarButton>
              {chnotType !== ChnotType.MarkdownWithToent && (
                <RadixPopover.Root>
                  <RadixPopover.Trigger>
                    <TopbarButton falseButton={true}>
                      <Icon.NotebookText className="w-4 h-4" />
                    </TopbarButton>
                  </RadixPopover.Trigger>

                  <RadixPopover.Portal>
                    <RadixPopover.Content
                      className="PopoverContent z-10 rounded-xl p-2 max-w-240 w-120"
                      sideOffset={5}
                    >
                      <MarkdownEditor
                        onContentChange={onChange}
                        height={200}
                        content={chnot?.record.content}
                        foldGutter={false}
                      />
                    </RadixPopover.Content>
                  </RadixPopover.Portal>
                </RadixPopover.Root>
              )}
            </div>
          )}
        </div>

        <div className="flex space-x-2 items-center">
          <div>{chnot?.record.insert_time.toLocaleDateString()}</div>
          {editState.requestState === RequestState.Requesting ? (
            <div className="flex items-center transition-opacity duration-300 ease-in-out opacity-100">
              <Icon.Loader2 className="animate-spin h-5 w-5" />
            </div>
          ) : editState.requestState === RequestState.Error ? (
            <div className="flex items-center text-red-600 transition-opacity duration-300 ease-in-out opacity-100">
              <Icon.LucideMessageCircleQuestion className="h-5 w-5" />
            </div>
          ) : (
            <div className="flex items-center transition-opacity duration-300 ease-in-out opacity-100">
              <Icon.CheckCircle className="h-5 w-5" />
            </div>
          )}
        </div>
      </div>
      <div className="h-full" ref={cmRef}>
        {chnotType === ChnotType.MarkdownWithToent &&
          (viewMode && chnot ? (
            <div className="p-2 overflow-y-auto">
              <MarkdownViewer
                content={chnot.record.content.replace("\n", "  \n")}
              />
            </div>
          ) : (
            <div
              className="h-full p-0 x-0 overflow-auto bg-editor" // this part could resize when I add overflow-auto, magic?
            >
              {height ? (
                <MarkdownEditor
                  onContentChange={onChange}
                  height={height}
                  content={chnot?.record.content}
                  foldGutter={true}
                />
              ) : (
                <div>Height is 0!</div>
              )}
            </div>
          ))}
        {chnotType === ChnotType.ExcalidrawV1 && chnot && (
          <ExcalidrawContainer instanceId={chnot.meta.id} />
        )}
      </div>
    </div>
  );
};
