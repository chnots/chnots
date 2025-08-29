import useResizeObserver from "@react-hook/resize-observer";
import { RefObject, useEffect, useRef, useState } from "react";
import MarkdownViewer from "../../chnot-markdown-viewer";
import { ChnotMeta, ChnotKind } from "../../../po";

import {
  chnotOverwriteMdwts,
  chnotTagNames,
  MdwtRecords,
  toentTodoEventGuess,
} from "@/krate/chnot/service";
import { CompletionContext, CompletionResult } from "@codemirror/autocomplete";
import { MdwtEditorMemo } from "@/common/component/codemirror/mdwt-editor";
import useDebounce from "@/hooks/use-debounce";
import { useChnotRopeStore } from "..";
import { SaveState } from "@/common/types";
import { ChnotMetaKind } from "../../vo";
import { TID } from "@/lib/id_util";
import { ChnotOverwriteMdwtReq } from "@/krate/chnot/dto";

const chnotCompletions = async (
  context: CompletionContext,
): Promise<CompletionResult | null> => {
  const word = context.matchBefore(/#[^# ]*|^#* \[|^[ ]*- \[|\[\[/);
  let options;
  if (!word || (word?.from == word?.to && !context.explicit)) {
    return null;
  } else if (word.text.startsWith("#")) {
    options = (
      await chnotTagNames({
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
  setSaveState,
  blockKindsRef,
}: {
  otid: TID;
  isFocused?: boolean;
  setSaveState: (saveState: SaveState) => void;
  blockKindsRef: RefObject<Map<TID, ChnotMetaKind>>;
}) => {
  const bodyRef = useRef<HTMLDivElement>(null);
  const [height, setHeight] = useState<number | undefined>(undefined);
  useResizeObserver<HTMLDivElement>(bodyRef, (entry) => {
    setHeight(entry.contentRect.height);
  });
  const [content, setContent] = useState<string>();
  const savedContentRef = useRef<string>(null);

  useEffect(() => {
    const mdwt_otid = otid;
    MdwtRecords({
      mdwt_otids: [mdwt_otid],
    }).then((rsp) => {
      const mdwt = rsp.mdwt_map[mdwt_otid];
      savedContentRef.current = mdwt?.content;
      setContent(mdwt?.content);
    });
  }, []);

  const { chnotMetaOtid } = useChnotRopeStore((state) => {
    return {
      chnotMetaOtid: state.chnotMetaOtid,
    };
  });

  const debounceSave = useDebounce(
    async (req: ChnotOverwriteMdwtReq) => {
      try {
        setSaveState(SaveState.Saving);
        const rsp = await chnotOverwriteMdwts(req);
        blockKindsRef.current.set(otid, {
          otid: otid,
          kind: ChnotKind.MarkdownWithToent,
          kind_id: otid.toString(),
        });
        setSaveState(SaveState.Saved);
      } catch (ex) {
        setSaveState(SaveState.Error);
      }
    },
    1000,
    true,
  );
  useEffect(() => {
    if (!content || savedContentRef.current === content) {
      return;
    }
    setSaveState(SaveState.Dirty);
    const req: ChnotOverwriteMdwtReq = {
      thread_otid: chnotMetaOtid,
      mdwts: [
        {
          otid: otid,
          content: content,
        },
      ],
    };

    debounceSave(req);
  }, [content]);

  return !isFocused && content ? (
    <MarkdownViewer content={content ?? ""} keepBreak={true} />
  ) : (
    <div className="w-full break-all">
      <MdwtEditorMemo
        content={content}
        onContentChange={(content) => {
          setContent(content);
        }}
        autoCompletion={chnotCompletions}
        height={height}
        foldGutter={false}
      />
    </div>
  );
};

export default MdwtRecord;
