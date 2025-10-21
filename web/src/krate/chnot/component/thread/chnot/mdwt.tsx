import useResizeObserver from "@react-hook/resize-observer";
import { useEffect, useRef, useState } from "react";
import MarkdownViewer from "../../chnot-markdown-viewer";
import { ChnotKind } from "../../../po";

import { CompletionContext, CompletionResult } from "@codemirror/autocomplete";
import { MdwtEditorMemo } from "@/krate/mdwt/component/codemirror/mdwt-editor";
import useDebounce from "@/hooks/use-debounce";
import { SaveState } from "@/common/types";
import { ChnotChromeProps } from "./rich-chnot";
import { genTID, TID } from "@/lib/id_util";
import {
  chnotTagNameList,
  mdwtCommit,
  mdwtRecordList,
} from "@/krate/mdwt/service";
import { MdwtCommitReq } from "@/krate/mdwt/dto";
import { toentTodoEventGuess } from "@/krate/toent/service";

const chnotCompletions = async (
  context: CompletionContext,
): Promise<CompletionResult | null> => {
  const word = context.matchBefore(/#[^# ]*|^#* \[|^[ ]*- \[|\[\[/);
  let options;
  if (!word || (word?.from == word?.to && !context.explicit)) {
    return null;
  } else if (word.text.startsWith("#")) {
    options = (
      await chnotTagNameList({
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

const MdwtRecord = ({ readonly, onPostSave, kindId }: ChnotChromeProps) => {
  const [mdwtOtid] = useState<TID>(kindId ? parseInt(kindId) : genTID());

  const bodyRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(undefined);
  useResizeObserver<HTMLDivElement>(bodyRef, (entry) => {
    setHeight(entry.contentRect.height);
  });

  // use RefObject to avoid
  const cachedContentRef = useRef<string>("");
  const saveStateRef = useRef<SaveState>(SaveState.Dirty);
  const toSaveArg = useRef<MdwtCommitReq>(null);
  const [refreshFlag, setRefreshFlag] = useState<boolean>();

  useEffect(() => {
    mdwtRecordList({
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
        onPostSave({
          saveState: SaveState.Saving,
          kindId: mdwtOtid.toString(),
        });
        await mdwtCommit(toSaveArg.current);
        onPostSave({
          saveState: SaveState.Saved,
          kindId: mdwtOtid.toString(),
        });
        toSaveArg.current = null;
      } catch (_ex) {
        onPostSave({ saveState: SaveState.Error, kindId: mdwtOtid.toString() });
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

  return !readonly ? (
    <MarkdownViewer content={cachedContentRef.current ?? ""} keepBreak={true} />
  ) : (
    <div className="w-full h-full break-all" onBlur={() => directlySave()}>
      <MdwtEditorMemo
        content={cachedContentRef.current}
        onContentChange={(content) => {
          cachedContentRef.current = content;
          if (saveStateRef.current != SaveState.Dirty) {
            onPostSave({
              saveState: SaveState.Dirty,
              kindId: mdwtOtid.toString(),
            });
          }

          const req: MdwtCommitReq = {
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
