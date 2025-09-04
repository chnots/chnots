import useResizeObserver from "@react-hook/resize-observer";
import { useEffect, useRef, useState } from "react";
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
import { ChnotOverwriteMdwtReq } from "@/krate/chnot/dto";
import { ChnotChromeProps } from "./chrome";
import { genTID, TID } from "@/lib/id_util";

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
  chnotOtid,
  isFocused,
  onPostSave,
  kindId,
}: ChnotChromeProps) => {
  const [mdwtOtid] = useState<TID>(kindId ? parseInt(kindId) : genTID());

  const bodyRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(undefined);
  useResizeObserver<HTMLDivElement>(bodyRef, (entry) => {
    setHeight(entry.contentRect.height);
  });

  // use RefObject to avoid
  const cachedContentRef = useRef<string>("");
  const saveStateRef = useRef<SaveState>(SaveState.Dirty);
  const toSaveArg = useRef<ChnotOverwriteMdwtReq>(null);
  const [refreshFlag, setRefreshFlag] = useState<boolean>();

  useEffect(() => {
    MdwtRecords({
      mdwt_otids: [mdwtOtid],
    }).then((rsp) => {
      const mdwt = rsp.mdwt_map[mdwtOtid];
      cachedContentRef.current = mdwt?.content ?? "";
      setRefreshFlag((prev) => !prev);
    });
  }, [mdwtOtid]);

  const directlySave = async () => {
    if (toSaveArg.current) {
      try {
        onPostSave({ saveState: SaveState.Saving });
        await chnotOverwriteMdwts(toSaveArg.current);
        let first = toSaveArg.current.mdwt;
        onPostSave({
          content: first.content,
          saveState: SaveState.Saved,
          data: {
            chnotOtid: chnotOtid,
            kind: ChnotKind.MDWT,
            kindId: mdwtOtid.toString(),
          },
        });
        toSaveArg.current = null;
      } catch (_ex) {
        onPostSave({ saveState: SaveState.Error });
      }
    }
  };

  const debounceSave = useDebounce(
    async () => {
      directlySave();
    },
    5000,
    true,
  );

  return !isFocused ? (
    <MarkdownViewer content={cachedContentRef.current ?? ""} keepBreak={true} />
  ) : (
    <div className="w-full break-all" onBlur={() => directlySave()}>
      <MdwtEditorMemo
        content={cachedContentRef.current}
        onContentChange={(content) => {
          cachedContentRef.current = content;
          if (saveStateRef.current != SaveState.Dirty) {
            onPostSave({ saveState: SaveState.Dirty });
          }

          const req: ChnotOverwriteMdwtReq = {
            mdwt: {
              otid: mdwtOtid,
              content: content,
            },
          };
          toSaveArg.current = req;

          debounceSave();
        }}
        autoCompletion={chnotCompletions}
        height={height}
        foldGutter={false}
      />
    </div>
  );
};

export default MdwtRecord;
