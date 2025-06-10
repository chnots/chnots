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
import { ChnotKind } from "@/krate/chnot/store/db";
import ExcalidrawContainer from "@/krate/tool/excalidraw/component/excalidraw-container";
import { enumFromStringValue } from "@/lib/enum-util";

import { CommonKFile } from "@/krate/kfile/components/common-kfile";
import { insertKKV } from "@/krate/kfile/store/service";
import { KSpaceSelect } from "@/krate/kspace/component/kspace-select";
import { Button } from "@/common/component/ui/button";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/common/component/ui/popover";
import { PopoverAnchor } from "@radix-ui/react-popover";
import { Tabs, TabsList, TabsTrigger } from "@/common/component/ui/tabs";
import { Toggle } from "@/common/component/ui/toggle";
import { KTabMeta } from "@/krate/ktab/store/po";
import KTabChnot from "@/krate/ktab/component/ktab-container";
import SessionContainer from "@/krate/llmchat/component/session-container";
import { SidebarTrigger } from "@/common/component/ui/sidebar";

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
  console.log("render ChnotContainer");

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

  const [chnotType, setChnotType] = useState<ChnotKind>(
    enumFromStringValue(
      ChnotKind,
      chnot?.meta.kind,
      ChnotKind.MarkdownWithToent
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

  const onSubKindRelationPersist = useCallback(
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

  return (
    <div className={clsx(className, "flex flex-col h-full")}>
      <div className="w-full flex items-center border-b kc-basic-with-bdr px-3 justify-between text-xs align-middle">
        <div className="text-xs flex space-x-2 p-1 items-center">
          <SidebarTrigger />
          {onClickNewButton &&
            (chnot ? (
              <Button size="sm" onClick={() => onClickNewButton()}>
                <Icon.BadgePlus className="w-4 h-4" />
                <span>New</span>
              </Button>
            ) : (
              <Tabs defaultValue={chnotType}>
                <TabsList>
                  <TabsTrigger
                    onClick={() => {
                      setChnotType(ChnotKind.MarkdownWithToent);
                    }}
                    value={ChnotKind.MarkdownWithToent}
                  >
                    <Icon.TextCursor className="w-4 h-4" />
                  </TabsTrigger>
                  <TabsTrigger
                    onClick={() => {
                      setChnotType(ChnotKind.ExcalidrawV1);
                    }}
                    value={ChnotKind.ExcalidrawV1}
                  >
                    <Icon.Pen className="w-4 h-4" />
                  </TabsTrigger>
                  <TabsTrigger
                    onClick={() => {
                      setChnotType(ChnotKind.KFileV1);
                    }}
                    value={ChnotKind.KFileV1}
                  >
                    <Icon.File className="w-4 h-4" />
                  </TabsTrigger>
                  <TabsTrigger
                    onClick={() => {
                      setChnotType(ChnotKind.KTab);
                    }}
                    value={ChnotKind.KTab}
                  >
                    <Icon.Table className="w-4 h-4" />
                  </TabsTrigger>
                  <TabsTrigger
                    onClick={() => {
                      setChnotType(ChnotKind.LLMChat);
                    }}
                    value={ChnotKind.LLMChat}
                  >
                    <Icon.Bot className="w-4 h-4" />
                  </TabsTrigger>
                </TabsList>
              </Tabs>
            ))}
          {chnot && (
            <div className="flex space-x-2">
              <KSpaceSelect
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
                onlyIcon={true}
              />
              <Toggle
                onClick={() => {
                  if (globalViewMode) {
                    globalViewMode.current = !viewMode;
                  }
                  setViewMode((prev) => !prev);
                }}
              >
                <Icon.Eye className="w-4 h-4" />
              </Toggle>
              {chnotType !== ChnotKind.MarkdownWithToent && (
                <Popover>
                  <PopoverTrigger>
                    <Button variant={"ghost"}>
                      <Icon.NotebookText className="w-4 h-4" />
                    </Button>
                  </PopoverTrigger>

                  <PopoverAnchor>
                    <PopoverContent
                      className="PopoverContent z-10 rounded-xl p-2 max-w-240 w-120"
                      sideOffset={5}
                    >
                      <MarkdownEditor
                        onContentChange={onChange}
                        height={200}
                        content={chnot?.record.content}
                        foldGutter={false}
                      />
                    </PopoverContent>
                  </PopoverAnchor>
                </Popover>
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
        className="h-full w-full flex justify-center overflow-auto content-centere"
        ref={cmRef}
      >
        {chnotType === ChnotKind.MarkdownWithToent &&
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
        {chnotType === ChnotKind.ExcalidrawV1 && (
          <ExcalidrawContainer
            chnotMetaId={chnot?.meta.id}
            afterSaveCallback={(id) => {
              onSubKindRelationPersist(
                `# Excalidraw ${new Date().toLocaleTimeString()}\n\n${
                  listViewTypeGetTagPath(listViewType) ?? ""
                }`,
                id
              );
            }}
          />
        )}
        {chnotType === ChnotKind.KFileV1 && (
          <CommonKFile
            chnotMetaId={chnot?.meta.id}
            onSave={(r) => {
              onSubKindRelationPersist(
                `# ${r.ori_filename}\n\n${
                  listViewTypeGetTagPath(listViewType) ?? ""
                }`,
                r.id
              );
            }}
          />
        )}
        {chnotType === ChnotKind.KTab && (
          <KTabChnot
            chnotMetaId={chnot?.meta.id}
            onInitialSave={async (meta: KTabMeta) => {
              onSubKindRelationPersist(
                `# Table ${meta.table_name}\n\n${meta.table_comment}\n\n ${
                  listViewTypeGetTagPath(listViewType) ?? ""
                }`,
                meta.id.toString()
              );
            }}
            kspace={currentKSpace.name}
            isEditing={!viewMode}
          />
        )}
        {chnotType === ChnotKind.LLMChat && (
          <div className="w-full">
            <SessionContainer
              chnotMetaId={chnot?.meta.id}
              onNewButton={() => {}}
              afterInit={(session) => {
                onSubKindRelationPersist(
                  `# ${session.title}  \n\n ${
                    listViewTypeGetTagPath(listViewType) ?? ""
                  }`,
                  session.id
                );
              }}
              kspace={currentKSpace.name}
            />
          </div>
        )}
      </div>
    </div>
  );
};
