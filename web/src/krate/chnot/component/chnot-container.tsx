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
  chnotOverwrite,
  chnotQuery,
  chnotQueryKindRel,
  chnotUpdate,
} from "@/krate/chnot/service";
import MarkdownViewer from "./chnot-markdown-viewer";
import MarkdownEditor from "./chnot-markdown-editor";
import { ChnotKind } from "@/krate/chnot/po";
import ExcalidrawContainer from "@/krate/tool/excalidraw/component/excalidraw-container";

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
import { TID } from "@/lib/id_util";
import { CommonKFile } from "@/krate/kfile/components/common-kfile";
import KTabChnot from "@/krate/ktab/component/ktab-container";
import { KTabMeta } from "@/krate/ktab/po";
import SessionContainer from "@/krate/llmchat/component/session-container";
import { ChnotKindIcon } from "./chnot-kind-icon";
import { createStore, StoreApi, useStore } from "zustand";
import { useShallow } from "zustand/react/shallow";
import { Chnot, ChnotOverwriteReq } from "../dto";
import useDebounce from "@/hooks/use-debounce";
import LoadingPage from "@/common/pages/loading-page";

enum RequestState {
  Saved,
  Requesting,
  Error,
}

interface ChnotProps {
  metaTid?: TID;
  kind: ChnotKind;
  kspace: string;
  readonly: boolean;
  topleft: ReactNode;
  onClickNewButton: () => void;
  onSetReadonly: (readonly: boolean) => void;
  onChnotChange: (chnot: Chnot) => void;
}

interface ChnotState extends ChnotProps {
  recTid?: TID;
  isUploadingKFile: boolean;
  requestState: RequestState;
  isComposing: boolean;
  content?: string;
  kindId?: string;
  onSetRecTid: (recTid: TID) => void;
  onSetMetaTid: (metaTid: TID) => void;
  onSetKind: (kind: ChnotKind) => void;
  onSetKindId: (kindId: string) => void;
  onSetRequestState: (requestState: RequestState) => void;
  onSetContent: (content: string) => void;
}

function createChnotStore(props: ChnotProps) {
  return createStore<ChnotState>()((set) => ({
    ...props,
    isUploadingKFile: false,
    requestState: RequestState.Saved,
    isComposing: false,
    onSetKind: (kind: ChnotKind) => {
      set((prev) => {
        return { ...prev, kind: kind };
      });
    },
    onSetRequestState: (requestState: RequestState) => {
      set((prev) => {
        return { ...prev, requestState: requestState };
      });
    },
    onSetMetaTid: (metaTid) => {
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
  }));
}

const ChnotContext = createContext<StoreApi<ChnotState> | null>(null);

function useChnotComStore<T>(selector: (state: ChnotState) => T) {
  const store = useContext(ChnotContext);

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
  props: ChnotProps;
  children: React.ReactNode;
}) {
  const [store] = useState<StoreApi<ChnotState>>(createChnotStore(props));

  return store ? (
    <ChnotContext.Provider value={store}>{children}</ChnotContext.Provider>
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
    onSetRequestState,
    onSetMetaTid,
    onSetRecTid,
    requestState,
    onChnotChange,
    kind,
  } = useChnotComStore((store) => {
    return {
      content: store.content,
      chnotKind: store.kind,
      metaTid: store.metaTid,
      kindId: store.kindId,
      kind: store.kind,
      onSetRequestState: store.onSetRequestState,
      onSetMetaTid: store.onSetMetaTid,
      onSetRecTid: store.onSetRecTid,
      requestState: store.requestState,
      onChnotChange: store.onChnotChange,
      
    };
  });
  const metaTidRef = useRef(metaTid);

  const debounceSave = useDebounce(
    async (req: ChnotOverwriteReq) => {
      onSetRequestState(RequestState.Requesting);
      const rsp = await chnotOverwrite(req);
      console.log("set Tid: ", rsp);
      if (!metaTidRef.current) {
        onSetMetaTid(rsp.meta_tid);
        metaTidRef.current = rsp.meta_tid;
      }
      onSetRecTid(rsp.rec_tid);
      onSetRequestState(RequestState.Saved);
      onChnotChange({
        record: {
          tid: rsp.rec_tid,
          meta_tid: rsp.meta_tid,
          content: req.content,
          archor: rsp.archor,
        },
        meta: {
          tid: rsp.meta_tid,
          kspace: rsp.kspace,
          kind: kind,
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

    const req: ChnotOverwriteReq = {
      content: content ?? "",
      kind: chnotKind!,
      meta_tid: metaTidRef.current,
      kind_id: kindId,
    };
    debounceSave(req);
  }, [content, chnotKind, kindId]);

  return requestState === RequestState.Requesting ? (
    <div className="flex items-center transition-opacity duration-300 ease-in-out opacity-100">
      <Icon.Loader2 className="animate-spin h-5 w-5" />
    </div>
  ) : requestState === RequestState.Error ? (
    <div className="flex items-center text-red-600 transition-opacity duration-300 ease-in-out opacity-100">
      <Icon.FileQuestion className="h-5 w-5" />
    </div>
  ) : (
    <div className="flex items-center transition-opacity duration-300 ease-in-out opacity-100">
      <Icon.CheckCircle className="h-5 w-5" />
    </div>
  );
};

const ChnotTopbar = () => {
  const {
    lefttop,
    kspace,
    readonly,
    recTid,
    content,
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
      kspace: store.kspace,
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

  const { listViewType } = useChnotStore(
    useShallow((store) => {
      return {
        overwriteChnot: store.appendChnot,
        listViewType: store.listViewType,
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
                {Object.values(ChnotKind).map((e) => (
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
            <KSpaceSelect
              onSelect={(ns) => {
                chnotUpdate({
                  meta_tid: metaTid,
                  kspace: ns,
                });
              }}
              currentKSpace={kspace}
              onlyIcon={true}
            />
            <Toggle
              onClick={() => {
                onSetReadonly(!readonly);
              }}
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
                          onSetContent(content + "\n" + (listViewType ?? ""));
                        }
                      }}
                      height={200}
                      content={content}
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
  readonly,
}: {
  metaTid?: TID;
  readonly: boolean;
  kind: ChnotKind;
}) => {
  const bodyRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(undefined);
  const [content, setContent] = useState<string | undefined>();
  useResizeObserver<HTMLDivElement>(bodyRef, (entry) => {
    setHeight(entry.contentRect.height);
  });

  const { chnotKind, onSetKindId, onSetContent } = useChnotComStore((store) => {
    return {
      chnotKind: store.kind,
      onSetKindId: store.onSetKindId,
      onSetContent: store.onSetContent,
    };
  });

  useEffect(() => {
    if (metaTid) {
      chnotQuery({
        view_type: {
          kind: "timeline",
        },
        kinds: [],
        start_index: 0,
        page_size: 1,
        meta_tid: metaTid,
      })
        .then((rsp) => {
          const chnot = rsp.data.at(0);
          setContent(chnot?.record.content);
        })
        .finally(() => {});
    }
  }, []);

  return (
    <div
      className="h-full w-full flex justify-center overflow-auto content-centere"
      ref={bodyRef}
    >
      {chnotKind === ChnotKind.MarkdownWithToent ? (
        readonly && content ? (
          <div className="p-2 overflow-y-auto w-full">
            <MarkdownViewer content={content.replace("\n", "  \n") ?? ""} />
          </div>
        ) : (
          <div
            className="h-full p-0 x-0 overflow-auto bg-editor w-full" // this part could resize when I add overflow-auto, magic?
          >
            {height ? (
              <MarkdownEditor
                onContentChange={(content) => {
                  onSetContent(content);
                }}
                height={height}
                content={content}
                foldGutter={true}
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

  return (
    <div className={clsx(className, "flex flex-col h-full")}>
      <ChnotTopbar />
      <ChnotBodyMemo readonly={readonly} kind={kind} metaTid={metaTid} />
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
}: {
  chnotKind: ChnotKind;
  metaTid?: TID;
  commonText: () => string;
  readOnly: boolean;
  onInitRel: (content: string, kind_id: string) => void;
}) => {
  const [kindId, setKindId] = useState<string>();
  console.log(`RichChnot: kindId: ${kindId}, metaTid ${metaTid}`);

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
                meta.tid.toString()
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
                  session.tid.toString()
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
export type { ChnotProps };
