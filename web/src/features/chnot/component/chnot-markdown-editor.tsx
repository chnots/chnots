import { useCallback, useEffect, useRef, useState } from "react";
import Icon from "@/common/component/icon";
import { CodeMirrorEditorMemo } from "@/common/component/codemirror-md-editor";
import useDebounce from "@/hooks/use-debounce";
import { useNamespaceStore } from "@/store/namespace";
import { NamespaceSelect } from "@/common/component/namespace-select";
import clsx from "clsx";
import useResizeObserver from "@react-hook/resize-observer";
import { Chnot, ChnotOverwriteReq } from "@/store/chnot/dto";
import { useChnotStore } from "@/store/chnot/store";
import { chnotTagNames, chnotUpdate, toentGuess } from "@/store/chnot/service";
import { CompletionContext, CompletionResult } from '@codemirror/autocomplete';
import { Separator } from "@radix-ui/react-separator";

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
    options = (await chnotTagNames({
      query: word.text, start_index: 0, page_size: 20,
      query_type: "fuzzy"
    })).data.map((name) => { return { "label": name, "type": "hashtag" } })
  } else if (word.text.startsWith("<")) {
    options = (await toentGuess({ input: word.text.replace("<", "") })).toents.map((toent) => { return { "label": `<${toent.event}>`, "type": "hashtag" } })
  } else {
    return null;
  }

  options.sort((e1, e2) => e1.label.length - e2.label.length)

  return {
    from: word.from,
    options: options,
    filter: false
  }
}

export const ChnotMarkdownEditor = ({ chnot, className, chnotChange }: { className?: string, chnot?: Chnot; chnotChange?: (chnot: Chnot) => void }) => {
  const { currentNamespace } = useNamespaceStore();
  const {
    overwriteChnot,
    validateChnotCache,
  } = useChnotStore();

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
          kind: "mdwt",
        };

        const rsp = await overwriteChnot(req, true);
        if (chnotChange) {
          chnotChange(rsp.chnot);
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
    [setEditState, editState, chnotChange, chnot]
  );



  const onChange = useDebounce((content: string) => {
    saveContent(content);
  }, 1000, true);

  useEffect(() => {
    return () => {
      console.log("chnot-markdown-editor is destoryed")
    }
  }, []);

  return (
    <div className={clsx(className, "flex flex-col h-full shadow-lg border kborder pt-2 rounded-t-xl bg-secondary")}>
      <div className={"w-full flex items-center bg-secondary border-b kborder px-2 pb-1 justify-between"}>
        <div className="text-xs flex space-x-2 p-1 kborder bg-accent border rounded-xl">
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

          {chnot && (
            <NamespaceSelect
              className="w-5 h-5"
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
          )}
        </div>



        <div>{chnot?.record.insert_time.toLocaleDateString()}</div>
      </div>

      <div
        className="h-full p-0 x-0 overflow-auto bg-editor" // this part could resize when I add overflow-auto, magic?
        ref={cmRef}
      >
        {height && (
          <CodeMirrorEditorMemo
            content={chnot?.record.content}
            onContentChange={onChange}
            autoCompletion={chnotCompletions}
            height={height}
          />
        )}
      </div>
    </div>
  );
};
