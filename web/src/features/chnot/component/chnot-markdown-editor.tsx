import { v4 as uuid } from "uuid";
import { useCallback, useRef, useState } from "react";
import Icon from "@/common/component/icon";
import { CodeMirrorEditorMemo } from "@/common/component/codemirror-md-editor";
import useDebounce from "@/hooks/use-debounce";
import { useNamespaceStore } from "@/store/namespace";
import { NamespaceSelect } from "@/common/component/namespace-select";
import clsx from "clsx";
import useResizeObserver from "@react-hook/resize-observer";
import { ChnotOverwriteReq } from "@/store/chnot/dto";
import { useChnotStore } from "@/store/chnot/store";
import { chnotQuery, chnotTagNames, chnotUpdate, toentGuess } from "@/store/chnot/service";
import { CompletionContext, CompletionResult } from '@codemirror/autocomplete';
import { markdownLanguage } from "@codemirror/lang-markdown";

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


const chnotCompletions = async (context: CompletionContext): Promise<CompletionResult | null> => {
  const word = context.matchBefore(/#[^# ]*|<[^<>]*/)
  let options;
  if (!word || word?.from == word?.to && !context.explicit) {
    return null
  } else if (word.text.startsWith("#")) {
    options = (await chnotTagNames({ query: word.text, start_index: 0, page_size: 20 })).data.map((name) => { return { "label": name, "type": "hashtag" } })
  } else if (word.text.startsWith("<")) {
    options = (await toentGuess({ input: word.text.replace("<", "") })).toents.map((toent) => { return { "label": `<${toent.event}>`, "type": "hashtag" } })
  } else {
    return null;
  }

  options.sort((e1,e2)=> e1.label.length - e2.label.length)

  return {
    from: word.from,
    options: options,
    filter: false
  }
}

export const ChnotMarkdownEditor = ({ className }: { className?: string }) => {
  const { currentNamespace } = useNamespaceStore();
  const {
    currentChnotIndex,
    chnotMap,
    setCurrentChnot,
    overwriteChnot,
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

  const cmRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(undefined);
  useResizeObserver<HTMLDivElement>(cmRef, (entry) => {
    setHeight(entry.contentRect.height);
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

  const onChange = useDebounce((metaId: string, content: string) => {
    saveContent(metaId, content);
  }, 1000);

  const onChangeRef = useRef(onChange);



  const fetchContent = useCallback(async (id: string) => {
    const chnots = await chnotQuery({
      meta_id: id,
      start_index: 0,
      page_size: 1,
    });
    return chnots.data[0]?.record.content;
  }, []);

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
                chnotUpdate({
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
            onChangeRef={onChangeRef}
            id={currentChnot?.meta.id ?? uuid()}
            fetchDefaultValue={fetchContent}
            height={height}
            autoCompletion={chnotCompletions}
          />
        )}
      </div>
    </div>
  );
};
