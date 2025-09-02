import useResizeObserver from "@react-hook/resize-observer";
import { RefObject, useEffect, useRef, useState } from "react";
import MarkdownViewer from "../../chnot-markdown-viewer";
import { ChnotKind } from "../../../po";

import {
  chnotOverwriteMdwts,
  chnotThreadTagNames,
  MdwtRecords,
  toentTodoEventGuess,
} from "@/krate/chnot/service";
import { CompletionContext, CompletionResult } from "@codemirror/autocomplete";
import { MdwtEditorMemo } from "@/common/component/codemirror/mdwt-editor";
import useDebounce from "@/hooks/use-debounce";
import { SaveState } from "@/common/types";
import { ChnotMetaKind } from "../../vo";
import { TID } from "@/lib/id_util";
import { ChnotOverwriteMdwtReq } from "@/krate/chnot/dto";
import { PostSaveArg } from "./chrome";

const chnotCompletions = async (
  context: CompletionContext,
): Promise<CompletionResult | null> => {
  const word = context.matchBefore(/#[^# ]*|^#* \[|^[ ]*- \[|\[\[/);
  let options;
  if (!word || (word?.from == word?.to && !context.explicit)) {
    return null;
  } else if (word.text.startsWith("#")) {
    options = (
      await chnotThreadTagNames({
        query: word.text,
        start_index: 0,
        page_size: 20,
      })
    ).data.map((name) => {
      return { label: name, type: "hashtag" };
    });
  } else if (word.text.startsWith("[[")) {
    options = [{ label: `[[backlink-ph]]`, type: "backlink" }];
  } else if (word.text.includes("# [") || word.text.includes("- [")) {
    options = (
      await toentTodoEventGuess({ input: word.text.replace(/.*\[/, "") })
    ).toents.map((toent) => {
      return { label: `{${toent}}`, type: "toent" };
    });
  } else {
    return null;
  }

  options.sort((e1, e2) => e1.label.length - e2.label.length);

  return {
    from: word.from,
    options: options,
    filter: false,
  };
};

const MdwtRecord = ({
  otid,
  isFocused,
  onPostSave,
  blockKindsRef,
}: {
  otid: TID;
  isFocused?: boolean;
  onPostSave: (arg: PostSaveArg) => void;
  blockKindsRef: RefObject<Map<TID, ChnotMetaKind>>;
}) => {
  console.log("rerender mdwt ", otid);
  const bodyRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(undefined);
  useResizeObserver<HTMLDivElement>(bodyRef, (entry) => {
    setHeight(entry.contentRect.height);
  });
  const [content, setContent] = useState<string>("");
  const cachedContentRef = useRef<string>("");
  const saveStateRef = useRef<SaveState>(SaveState.Dirty);

  useEffect(() => {
    const mdwt_otid = otid;
    MdwtRecords({
      mdwt_otids: [mdwt_otid],
    }).then((rsp) => {
      const mdwt = rsp.mdwt_map[mdwt_otid];
      cachedContentRef.current = mdwt?.content ?? "";
      setContent(mdwt.content);
    });
  }, []);

  const debounceSave = useDebounce(
    async (req: ChnotOverwriteMdwtReq) => {
      try {
        onPostSave({ saveState: SaveState.Saving });
        await chnotOverwriteMdwts(req);
        blockKindsRef.current.set(otid, {
          otid: otid,
          kind: ChnotKind.MDWT,
          kind_id: otid.toString(),
        });
        let first = req.mdwt;
        onPostSave({
          otid: first.otid,
          kind: ChnotKind.MDWT,
          content: first.content,
          saveState: SaveState.Saved,
        });
      } catch (_ex) {
        onPostSave({ saveState: SaveState.Error });
      }
    },
    1000,
    true,
  );

  return !isFocused ? (
    <MarkdownViewer content={content ?? ""} keepBreak={true} />
  ) : (
    <div
      className="w-full break-all"
      onBlur={() => {
        setContent(cachedContentRef.current);
      }}
    >
      <MdwtEditorMemo
        content={content}
        onContentChange={(content) => {
          cachedContentRef.current = content;
          if (saveStateRef.current != SaveState.Dirty) {
            onPostSave({ saveState: SaveState.Dirty });
          }

          const req: ChnotOverwriteMdwtReq = {
            mdwt: {
              otid: otid,
              content: content,
            },
          };

          debounceSave(req);
        }}
        autoCompletion={chnotCompletions}
        height={height}
        foldGutter={false}
      />
    </div>
  );
};

export default MdwtRecord;
