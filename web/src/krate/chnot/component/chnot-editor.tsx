import React, {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";
import Icon from "@/common/component/icon";
import clsx from "clsx";
import useResizeObserver from "@react-hook/resize-observer";
import { useChnotStore } from "@/krate/chnot/store";
import {
  chnotOverwriteRecord,
  chnotQuery,
  chnotQueryKindRel,
} from "@/krate/chnot/service";
import MarkdownViewer from "./chnot-markdown-viewer";
import MarkdownEditor from "./chnot-markdown-editor";
import { ChnotKind } from "@/krate/chnot/po";
import ExcalidrawContainer from "@/krate/tool/excalidraw/component/excalidraw-container";

import { Button } from "@/common/component/ui/button";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/common/component/ui/popover";
import { PopoverAnchor } from "@radix-ui/react-popover";
import { Tabs, TabsList, TabsTrigger } from "@/common/component/ui/tabs";
import { Toggle } from "@/common/component/ui/toggle";
import { TID } from "@/lib/id_util";
import { CommonKFile } from "@/krate/kfile/components/common-kfile";
import KTabChnot from "@/krate/ktab/component/ktab-container";
import { KTabMeta } from "@/krate/ktab/po";
import SessionContainer from "@/krate/llmchat/component/session-container";
import { ChnotKindIcon } from "./chnot-kind-icon";
import { createStore, StoreApi, useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";
import { Chnot, ChnotOverwriteRecordReq } from "../dto";
import useDebounce from "@/hooks/use-debounce";
import LoadingPage from "@/common/pages/loading-page";
import { SaveState } from "@/common/types";

interface ChnotEditorProps {
  metaTid?: TID;
  kind: ChnotKind;
  readonly: boolean;
  topleft: ReactNode;
  onClickNewButton: () => void;
  onSetReadonly: (readonly: boolean) => void;
  onChnotChange: (chnot: Chnot) => void;
  onSetMetaTid: (metaTid: TID) => void;
}

interface ChnotEditorState extends ChnotEditorProps {
  recTid?: TID;
  isUploadingKFile: boolean;
  saveState: SaveState;
  isComposing: boolean;
  content?: string;
  kindId?: string;
  onSetRecTid: (recTid: TID) => void;
  onSetKind: (kind: ChnotKind) => void;
  onSetKindId: (kindId: string) => void;
  onSetSaveState: (saveState: SaveState) => void;
  onSetContent: (content: string) => void;
}

function createChnotStore(props: ChnotEditorProps) {
  return createStore<ChnotEditorState>()((set) => ({
    ...props,
    isUploadingKFile: false,
    saveState: SaveState.Saved,
    isComposing: false,
    onSetKind: (kind: ChnotKind) => {
      set((prev) => {
        return { ...prev, kind: kind };
      });
    },
    onSetSaveState: (saveState: SaveState) => {
      set((prev) => {
        return { ...prev, saveState: saveState };
      });
    },
    onSetMetaTid: (metaTid) => {
      props.onSetMetaTid(metaTid);
      set((prev) => {
        console.log("tids:", metaTid);
        return { ...prev, metaTid: metaTid };
      });
    },
    onSetRecTid: (recTid) => {
      set((prev) => {
        console.log("tids:", recTid);
        return { ...prev, recTid: recTid };
      });
    },
    onSetContent: (content: string) => {
      set((prev) => {
        return { ...prev, content: content };
      });
    },
    onSetKindId: (kindId) => {
      set((prev) => {
        return { ...prev, kindId: kindId };
      });
    },
    onSetReadonly: (readonly) => {
      console.log("set read only", readonly);
      props.onSetReadonly(readonly);
      set((prev) => {
        return { ...prev, readonly: readonly };
      });
    },
  }));
}

const ChnotEditorContext = createContext<StoreApi<ChnotEditorState> | null>(
  null
);

function useChnotComStore<T>(selector: (state: ChnotEditorState) => T) {
  const store = useContext(ChnotEditorContext);

  return useStore(
    store!,
    useShallow((store) => {
      return selector(store);
    })
  );
}

function ChnotEditorProvider({
  props,
  children,
}: {
  props: ChnotEditorProps;
  children: React.ReactNode;
}) {
  const [store] = useState<StoreApi<ChnotEditorState>>(createChnotStore(props));

  return store ? (
    <ChnotEditorContext.Provider value={store}>
      {children}
    </ChnotEditorContext.Provider>
  ) : (
    <LoadingPage />
  );
}

const ChnotSaver = () => {
  const {
    content,
    metaTid,
    chnotKind,
    kindId,
    onSetSaveState,
    onSetMetaTid,
    onSetRecTid,
    saveState,
    onChnotChange,
    kind,
  } = useChnotComStore((store) => {
    return {
      content: store.content,
      chnotKind: store.kind,
      metaTid: store.metaTid,
      kindId: store.kindId,
      kind: store.kind,
      onSetSaveState: store.onSetSaveState,
      onSetMetaTid: store.onSetMetaTid,
      onSetRecTid: store.onSetRecTid,
      saveState: store.saveState,
      onChnotChange: store.onChnotChange,
    };
  });
  const metaTidRef = useRef(metaTid);

  const debounceSave = useDebounce(
    async (req: ChnotOverwriteRecordReq) => {
      onSetSaveState(SaveState.Saving);
      const rsp = await chnotOverwriteRecord(req);
      if (!metaTidRef.current) {
        onSetMetaTid(rsp.meta_otid);
        metaTidRef.current = rsp.meta_otid;
      }
      onSetRecTid(rsp.rec_tid);
      onSetSaveState(SaveState.Saved);
      onChnotChange({
        record: {
          tid: rsp.rec_tid,
          meta_otid: rsp.meta_otid,
          content: req.content,
          archor: rsp.archor,
          todo_event: rsp.todo_event,
        },
        meta: {
          otid: rsp.meta_otid,
          kspace: rsp.kspace,
          kind: kind,
          tid: rsp.rec_tid,
        },
      });
    },
    1000,
    true
  );
  useEffect(() => {
    if (!content) {
      return;
    }
    onSetSaveState(SaveState.Dirty);
    const req: ChnotOverwriteRecordReq = {
      content: content ?? "",
      kind: chnotKind!,
      meta_otid: metaTidRef.current,
      kind_id: kindId,
    };

    debounceSave(req);
  }, [content, chnotKind, kindId]);

  return saveState === SaveState.Saving ? (
    <div className="flex items-center transition-opacity duration-300 ease-in-out opacity-100">
      <Icon.Loader2 className="animate-spin h-5 w-5" />
    </div>
  ) : saveState === SaveState.Saved ? (
    <div className="flex items-center text-green-600 transition-opacity duration-300 ease-in-out opacity-100">
      <Icon.Sun className="h-5 w-5" />
    </div>
  ) : saveState === SaveState.Dirty ? (
    <div className="flex items-center transition-opacity duration-300 ease-in-out opacity-100">
      <Icon.CloudOff className="h-5 w-5" />
    </div>
  ) : (
    <div className="flex items-center text-red-600 transition-opacity duration-300 ease-in-out opacity-100">
      <Icon.CloudAlert className="h-5 w-5" />
    </div>
  );
};

const ChnotTopbar = ({ initialContent }: { initialContent: string }) => {
  const {
    content,
    lefttop,
    readonly,
    recTid,
    metaTid,
    chnotKind,
    onSetKind,
    onSetReadonly,
    onClickNewButton,
    onSetContent,
  } = useChnotComStore((store) => {
    return {
      content: store.content,
      lefttop: store.topleft,
      readonly: store.readonly,
      recTid: store.recTid,
      chnotKind: store.kind,
      metaTid: store.metaTid,
      onSetKind: store.onSetKind,
      onSetReadonly: store.onSetReadonly,
      onClickNewButton: store.onClickNewButton,
      onSetContent: store.onSetContent,
    };
  });

  const { tags, kinds } = useChnotStore(
    useShallow((store) => {
      return {
        tags: store.tags,
        kinds: store.kinds,
      };
    })
  );

  return (
    <div className="w-full flex items-center border-b kc-basic-with-bdr px-3 justify-between text-xs align-middle">
      <div className="text-xs flex space-x-2 p-1 items-center">
        {lefttop}
        {onClickNewButton &&
          (metaTid ? (
            <Button size="sm" onClick={() => onClickNewButton()}>
              <Icon.BadgePlus className="w-4 h-4" />
              <span>New</span>
            </Button>
          ) : (
            <Tabs defaultValue={chnotKind}>
              <TabsList>
                {(kinds && kinds.length > 0
                  ? kinds
                  : Object.values(ChnotKind)
                ).map((e) => (
                  <TabsTrigger
                    key={e}
                    onClick={() => {
                      onSetKind(e);
                    }}
                    value={e}
                  >
                    <ChnotKindIcon className="w-4 h-4" kind={e} />
                  </TabsTrigger>
                ))}
              </TabsList>
            </Tabs>
          ))}
        {metaTid && (
          <div className="flex space-x-2">
            <Toggle
              onClick={() => {
                onSetReadonly(!readonly);
              }}
              defaultPressed={readonly}
            >
              <Icon.Eye className="w-4 h-4" />
            </Toggle>
            {chnotKind !== ChnotKind.MarkdownWithToent && (
              <Popover>
                <PopoverTrigger asChild>
                  <Button variant={"ghost"}>
                    <Icon.NotebookText className="w-4 h-4" size={4} />
                  </Button>
                </PopoverTrigger>

                <PopoverAnchor>
                  <PopoverContent
                    className="PopoverContent z-10 rounded-md p-2 max-w-240 w-120 border"
                    sideOffset={5}
                  >
                    <MarkdownEditor
                      onContentChange={(content) => {
                        if (metaTid) {
                          onSetContent(content);
                        } else {
                          onSetContent(content + "\n" + (tags ?? ""));
                        }
                      }}
                      height={200}
                      content={content ?? initialContent}
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
        <div>{recTid && new Date(recTid / 1e3).toLocaleDateString()}</div>
        <ChnotSaver />
      </div>
    </div>
  );
};

const ChnotBody = ({
  metaTid,
  initialContent,
  readonly,
}: {
  metaTid?: TID;
  initialContent?: string;
  readonly: boolean;
  kind: ChnotKind;
}) => {
  const bodyRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(undefined);
  useResizeObserver<HTMLDivElement>(bodyRef, (entry) => {
    setHeight(entry.contentRect.height);
  });

  const { chnotKind, onSetKindId, onSetContent, onSetSaveState } =
    useChnotComStore((store) => {
      return {
        chnotKind: store.kind,
        onSetKindId: store.onSetKindId,
        onSetContent: store.onSetContent,
        onSetSaveState: store.onSetSaveState,
      };
    });

  return (
    <div
      className="h-full w-full flex justify-center overflow-auto content-centere"
      ref={bodyRef}
    >
      {chnotKind === ChnotKind.MarkdownWithToent ? (
        readonly && initialContent ? (
          <div className="p-2 overflow-y-auto w-full">
            <MarkdownViewer
              content={initialContent.replace("\n", "  \n") ?? ""}
            />
          </div>
        ) : (
          <div
            className="h-full p-4 x-0 overflow-auto bg-editor w-full" // this part could resize when I add overflow-auto, magic?
          >
            {height ? (
              <MarkdownEditor
                onContentChange={(content) => {
                  onSetContent(content);
                }}
                height={height}
                content={initialContent}
                foldGutter={false}
              />
            ) : (
              <div />
            )}
          </div>
        )
      ) : (
        <RichChnot
          readOnly={readonly ?? false}
          metaTid={metaTid}
          chnotKind={chnotKind}
          commonText={() => ""}
          onInitRel={function (content: string, kind_id: string): void {
            if (!metaTid) {
              onSetContent(content);
            }
            onSetKindId(kind_id);
          }}
          onSetSaveState={onSetSaveState}
        />
      )}
    </div>
  );
};
const ChnotBodyMemo = React.memo(ChnotBody);

const ChnotEditor = ({ className }: { className?: string }) => {
  const { kind, readonly, metaTidStore } = useChnotComStore((store) => {
    return {
      metaTidStore: store.metaTid,
      kind: store.kind,
      readonly: store.readonly,
    };
  });

  const [metaTid] = useState(metaTidStore);
  const [content, setContent] = useState<string | undefined>();
  useEffect(() => {
    if (metaTid) {
      chnotQuery({
        kinds: [],
        start_index: 0,
        page_size: 1,
        meta_otid: metaTid,
      })
        .then((rsp) => {
          const chnot = rsp.data.at(0);
          setContent(chnot?.record.content);
        })
        .finally(() => {});
    }
  }, []);

  return (
    <div className={clsx(className, "flex flex-col h-full")}>
      <ChnotTopbar initialContent={content ?? ""} />
      <ChnotBodyMemo
        readonly={readonly}
        kind={kind}
        metaTid={metaTid}
        initialContent={content}
      />
    </div>
  );
};

/**
 * load kindId
 * @param param0
 * @returns
 */
const RichChnot = ({
  chnotKind,
  metaTid,
  commonText,
  readOnly,
  onInitRel,
  onSetSaveState,
}: {
  chnotKind: ChnotKind;
  metaTid?: TID;
  commonText: () => string;
  readOnly: boolean;
  onInitRel: (content: string, kind_id: string) => void;
  onSetSaveState: (state: SaveState) => void;
}) => {
  const [kindId, setKindId] = useState<string>();

  useEffect(() => {
    if (metaTid)
      chnotQueryKindRel(metaTid).then((rsp) => {
        setKindId(rsp.kind_rel.kind_id);
      });
  }, [metaTid]);

  const kindIdRef = useRef<string>(null);
  const handleSaveRel = (content: string, kind_id: string) => {
    if (kind_id !== kindIdRef.current) {
      onInitRel(content, kind_id);
      kindIdRef.current = kind_id;
    }
  };

  return (
    (kindId || !metaTid) && (
      <>
        {chnotKind === ChnotKind.ExcalidrawV1 ? (
          <ExcalidrawContainer
            kindId={kindId}
            onAfterSave={(tid) => {
              handleSaveRel(
                `# Excalidraw ${new Date().toLocaleTimeString()}\n\n${commonText()}`,
                tid.toString()
              );
            }}
            readOnly={readOnly}
            onSetSaveState={(state: SaveState) => {
              onSetSaveState(state);
            }}
          />
        ) : chnotKind === ChnotKind.KFileV1 ? (
          <CommonKFile
            kindId={kindId}
            onAfterSave={(r) => {
              handleSaveRel(`# ${r.filename}\n\n${commonText()}`, r.id);
            }}
          />
        ) : chnotKind === ChnotKind.KTab ? (
          <KTabChnot
            kindId={kindId}
            onAfterSave={async (meta: KTabMeta) => {
              handleSaveRel(
                `# Table ${meta.table_name}\n\n${
                  meta.table_comment
                }\n\n ${commonText()}`,
                meta.otid.toString()
              );
            }}
            isEditing={!readOnly}
          />
        ) : chnotKind === ChnotKind.LLMChat ? (
          <div className="w-full">
            <SessionContainer
              kindId={kindId}
              onAfterSave={(session) => {
                handleSaveRel(
                  `# LLM ${session.title}  \n\n ${commonText()}`,
                  session.otid.toString()
                );
              }}
            />
          </div>
        ) : (
          <div />
        )}
      </>
    )
  );
};

export { ChnotEditorProvider, ChnotEditor };
export type { ChnotEditorProps as ChnotProps };
