import { v4 as uuid } from "uuid";
import { useCallback, useRef, useState } from "react";
import { ChnotOverwriteReq } from "@/store/chnot";
import { useChnotStore } from "@/store/chnot";
import Icon from "@/common/component/icon";
import { CodeMirrorEditorMemo } from "@/common/component/codemirror-md-editor";
import useDebounce from "@/hooks/use-debounce";
import { useNamespaceStore } from "@/store/namespace";
import { NamespaceSelect } from "@/common/component/namespace-select";
import clsx from "clsx";
import useResizeObserver from "@react-hook/resize-observer";
import { observe } from "react-intersection-observer";

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

export const ChnotMarkdownEditor = ({ className }: { className?: string }) => {
  const { currentNamespace } = useNamespaceStore();
  const {
    currentChnotIndex,
    chnotMap,
    setCurrentChnot,
    overwriteChnot,
    queryChnot,
    updateChnot,
    validateChnotCache,
  } = useChnotStore();

  const currentChnot = currentChnotIndex
    ? chnotMap.get(currentChnotIndex)
    : undefined;

  const [editState, setEditState] = useState<ChnotEditState>({
    isUploadingResource: false,
    requestState: RequestState.Saved,
    isComposing: false,
  });

  const saveContent = useCallback(
    async (metaId: string, content?: string) => {
      setEditState((state) => {
        return {
          ...state,
          requestState: RequestState.Requesting,
        };
      });

      const req: ChnotOverwriteReq = {
        chnot: {
          id: uuid(),
          content: content ?? "",
          insert_time: new Date(),
          meta_id: metaId,
        },
        kind: "mdwt",
      };
      let requestState;
      try {
        const rsp = await overwriteChnot(req, true);
        setCurrentChnot(rsp.chnot);
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
    [setCurrentChnot, setEditState, editState]
  );

  const onChangeCallback = useCallback(() => {
    useDebounce((metaId: string, content: string) => {
      console.log("begin to save ", content);
      saveContent(metaId, content);
    }, 1000);
  }, []);

  const cmRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(undefined);
  useResizeObserver<HTMLDivElement>(cmRef, (entry) => {
    setHeight(entry.contentRect.height);
  });

  const fetchContent = useCallback(
    async (id: string) => {
      const chnot = await queryChnot({ meta_id: id });
      return chnot?.record.content;
    },
    [queryChnot]
  );

  return (
    <div className={clsx(className, "p-1 flex flex-col h-full")}>
      <div className={"w-full flex-row flex space-x-2"}>
        <div className="text-xs">
          {editState.requestState === RequestState.Requesting ? (
            <div className="flex items-center transition-opacity duration-300 ease-in-out opacity-100">
              <Icon.Loader2 className="animate-spin h-5 w-5 mr-2" />
            </div>
          ) : editState.requestState === RequestState.Error ? (
            <div className="flex items-center text-red-600 transition-opacity duration-300 ease-in-out opacity-100">
              <Icon.LucideMessageCircleQuestion className="h-5 w-5 mr-2" />
            </div>
          ) : (
            <div className="flex items-center transition-opacity duration-300 ease-in-out opacity-100">
              <Icon.CheckCircle className="h-5 w-5 mr-2" />
            </div>
          )}
        </div>
        {currentChnot && (
          <NamespaceSelect
            onSelect={(ns) => {
              if (currentChnot) {
                updateChnot({
                  meta_id: currentChnot.meta.id,
                  update_time: false,
                  namespace: ns,
                }).then((_) => {
                  if (ns !== currentNamespace.name) {
                    validateChnotCache([currentChnot.meta.id]);
                  }
                });
              }
            }}
            currentNamespace={currentNamespace.name}
          />
        )}
        <div>{currentChnot?.meta.insert_time.toDateString()}</div>
        <span>~</span>
        <div>{currentChnot?.record.insert_time.toDateString()}</div>
      </div>

      <div
        className="h-full border kborder shadow-lg p-0 x-0 overflow-auto" // this part could resize when I add overflow-auto, magic?
        ref={cmRef}
      >
        {height && (
          <CodeMirrorEditorMemo
            onChange={onChangeCallback}
            id={currentChnot?.meta.id ?? uuid()}
            fetchDefaultValue={fetchContent}
            height={height}
          />
        )}
      </div>
    </div>
  );
};
