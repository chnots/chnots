import React, { useCallback, useRef, useState } from "react";
import Icon from "@/common/component/icon";
import useDebounce from "@/hooks/use-debounce";
import { useKSpaceStore } from "@/krate/kspace/store/store";
import clsx from "clsx";
import useResizeObserver from "@react-hook/resize-observer";
import {
  Chnot,
  ChnotOverwriteReq,
  listViewTypeGetTagPath,
} from "@/krate/chnot/store/dto";
import { useChnotStore } from "@/krate/chnot/store/store";
import { chnotUpdate } from "@/krate/chnot/store/service";
import MarkdownViewer from "./chnot-markdown-viewer";
import MarkdownEditor from "./chnot-markdown-editor";
import { ChnotType } from "@/krate/chnot/store/db";
import ExcalidrawContainer from "@/krate/tool/excalidraw/component/excalidraw-container";
import KButton, { KButtonProps } from "@/common/component/kbutton";
import { enumFromStringValue } from "@/lib/enum-util";

import * as RadixPopover from "@radix-ui/react-popover";
import { CommonKFile } from "@/krate/kfile/components/common-kfile";
import { insertKKV } from "@/krate/kfile/store/service";
import { KSpaceSelect } from "@/krate/kspace/component/kspace-select";

enum RequestState {
  Saved,
  Requesting,
  Error,
}

interface ChnotEditState {
  isUploadingKFile: boolean;
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
  const { currentKSpace } = useKSpaceStore();
  const { overwriteChnot, validateChnotCache, listViewType } = useChnotStore();

  const [editState, setEditState] = useState<ChnotEditState>({
    isUploadingKFile: false,
    requestState: RequestState.Saved,
    isComposing: false,
  });

  const [viewMode, setViewMode] = useState(globalViewMode?.current ?? false);

  const cmRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(undefined);
  useResizeObserver<HTMLDivElement>(cmRef, (entry) => {
    setHeight(entry.contentRect.height);
  });

  const [chnotType, setChnotType] = useState<ChnotType>(
    enumFromStringValue(
      ChnotType,
      chnot?.meta.kind,
      ChnotType.MarkdownWithToent
    )!
  );

  const saveContent = useCallback(
    async (content?: string, subTypeId?: string) => {
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

        if (subTypeId) {
          await insertKKV({
            key: rsp.chnot.meta.id,
            value: subTypeId,
            kind: "chnot_sub_type",
          });
        }
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

  const onOtherTypeInit = useCallback(
    async (content: string, subTypeId: string) => {
      if (!chnot) {
        saveContent(content, subTypeId);
      }
    },
    [chnot, chnotType, saveContent]
  );

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
                  <Icon.Pen className="w-4 h-4" />
                </ChnotTypeButton>
                <ChnotTypeButton thisChnotType={ChnotType.KFileV1}>
                  <Icon.File className="w-4 h-4" />
                </ChnotTypeButton>
              </div>
            ))}
          {chnot && (
            <div className="bg-inactive border kc-active rounded-xl p-0.5 flex space-x-1">
              <KSpaceSelect
                className="w-4 h-4"
                menuClassName="px-1 py-1 bg-inactive rounded-xl flex items-center "
                onSelect={(ns) => {
                  chnotUpdate({
                    meta_id: chnot.meta.id,
                    update_time: false,
                    kspace: ns,
                  }).then((_) => {
                    if (ns !== currentKSpace.name) {
                      validateChnotCache([chnot.meta.id]);
                    }
                  });
                }}
                currentKSpace={chnot.meta.kspace}
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
      <div
        className="h-full w-full flex items-center justify-center align-middle overflow-auto content-center bg-active"
        ref={cmRef}
      >
        {chnotType === ChnotType.MarkdownWithToent &&
          (viewMode && chnot ? (
            <div className="p-2 overflow-y-auto w-full">
              <MarkdownViewer
                content={chnot.record.content.replace("\n", "  \n")}
              />
            </div>
          ) : (
            <div
              className="h-full p-0 x-0 overflow-auto bg-editor w-full" // this part could resize when I add overflow-auto, magic?
            >
              {height ? (
                <MarkdownEditor
                  onContentChange={onChange}
                  height={height}
                  content={chnot?.record.content}
                  foldGutter={true}
                />
              ) : (
                <div />
              )}
            </div>
          ))}
        {chnotType === ChnotType.ExcalidrawV1 && (
          <ExcalidrawContainer
            chnotMetaId={chnot?.meta.id}
            afterSaveCallback={(id) => {
              onOtherTypeInit(
                `# Excalidraw ${new Date().toLocaleTimeString()}\n\n${
                  listViewTypeGetTagPath(listViewType) ?? ""
                }`,
                id
              );
            }}
          />
        )}
        {chnotType === ChnotType.KFileV1 && (
          <CommonKFile
            chnotMetaId={chnot?.meta.id}
            onSave={(r) => {
              onOtherTypeInit(
                `# ${r.ori_filename}\n\n${
                  listViewTypeGetTagPath(listViewType) ?? ""
                }`,
                r.id
              );
            }}
          />
        )}
      </div>
    </div>
  );
};
