import { HEADING_OTID_RE, splitDocumentByBlocks } from "@chnots/md-codemirror";
import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import { Plus } from "lucide-react";
import React, { useCallback, useEffect, useRef, useState } from "react";
import { SaveState } from "@/common/types";
import MdwtEditor from "@/krate/mdwt/component/mdwt-editor";
import { mdwtCommit, mdwtRecordList } from "@/krate/mdwt/service";
import { arraysAreEqual } from "@/lib/col-util";
import { genTID, type TID } from "@/lib/id_util";
import { ChnotKind } from "../../../po";
import {
  chnotThreadMetaFetch,
  chnotThreadOrderArchive,
  chnotThreadOrderCommit,
} from "../../../service";
import MdwtChnot from "../mdwt";
import type { PostSaveArg, RichPropProps } from "../rich-mdwt-side";

function joinMdwtBlocks(
  blocks: Array<{ otid: number; content: string }>,
): string {
  return blocks
    .map((b) => {
      const lines = b.content.split("\n");
      const firstLine = lines[0] || "";
      if (HEADING_OTID_RE.test(firstLine)) {
        return b.content;
      }
      const cleaned = firstLine.replace(/^#{1,6}\s+/, "");
      const rest = lines.slice(1);
      return [`## [[${b.otid}]] ${cleaned}`, ...rest].join("\n");
    })
    .join("\n\n");
}

const ChnotThread = ({ otid: threadOtid, onPostSave }: RichPropProps) => {
  const cmRef = useRef<ReactCodeMirrorRef>(null);
  const savedBlockOtidsRef = useRef<number[]>([]);
  const lastSavedContentRef = useRef<Map<number, string>>(new Map());
  const saveTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const savingRef = useRef(false);
  const [joinedDoc, setJoinedDoc] = useState<string>("");
  const [threadContent, setThreadContent] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    (async () => {
      try {
        const rsp = await chnotThreadMetaFetch({ otid: threadOtid });
        const childOtids: TID[] = rsp.chnot_meta_sorted.map(
          (cm) => cm.meta.otid,
        );
        const mdwtRsp = await mdwtRecordList({
          mdwt_otids: [...childOtids, threadOtid],
        });

        setThreadContent(mdwtRsp.mdwt_map[threadOtid]?.content ?? "");

        const blocks = childOtids.map((otid) => ({
          otid,
          content: mdwtRsp.mdwt_map[otid]?.content ?? "",
        }));

        if (blocks.length > 0) {
          const doc = joinMdwtBlocks(blocks);
          setJoinedDoc(doc);
          savedBlockOtidsRef.current = childOtids;
        } else {
          const newOtid = genTID();
          const doc = `## [[${newOtid}]] `;
          setJoinedDoc(doc);
          savedBlockOtidsRef.current = [];
        }

        const savedMap = new Map<number, string>();
        for (const b of blocks) {
          const lines = b.content.split("\n");
          const firstLine = lines[0] || "";
          if (HEADING_OTID_RE.test(firstLine)) {
            savedMap.set(b.otid, b.content.trimEnd());
          } else {
            const cleaned = firstLine.replace(/^#{1,6}\s+/, "");
            const rest = lines.slice(1);
            const normalized = [`## [[${b.otid}]] ${cleaned}`, ...rest]
              .join("\n")
              .trimEnd();
            savedMap.set(b.otid, normalized);
          }
        }
        lastSavedContentRef.current = savedMap;
      } finally {
        setLoading(false);
      }
    })();
  }, [threadOtid]);

  const handlePostSave = useCallback(
    async (arg: PostSaveArg) => {
      await onPostSave({ ...arg, kind: ChnotKind.ThreadV1 });
    },
    [onPostSave],
  );

  const handleBlockSave = useCallback(async () => {
    if (savingRef.current) return;
    const view = cmRef.current?.view;
    if (!view) return;

    savingRef.current = true;
    try {
      const currentBlocks = splitDocumentByBlocks(view.state);
      const previousBlocks = lastSavedContentRef.current;
      const previousOtids = savedBlockOtidsRef.current;
      const currentOtids = [...currentBlocks.keys()];

      const added = currentOtids.filter((otid) => !previousBlocks.has(otid));
      const changed = currentOtids.filter(
        (otid) =>
          previousBlocks.has(otid) &&
          previousBlocks.get(otid) !== currentBlocks.get(otid),
      );
      const deleted = previousOtids.filter((otid) => !currentBlocks.has(otid));

      if (
        added.length > 0 ||
        deleted.length > 0 ||
        !arraysAreEqual(currentOtids, previousOtids, (a, b) => a === b)
      ) {
        await chnotThreadOrderCommit({
          thread_otid: threadOtid,
          orders: currentOtids.map((otid) => ({ otid, closed: false })),
        });
      }

      if (deleted.length > 0) {
        await chnotThreadOrderArchive({
          thread_otid: threadOtid,
          otids: deleted,
        });
      }

      for (const otid of [...added, ...changed]) {
        const content = currentBlocks.get(otid);
        if (content === undefined) continue;
        await mdwtCommit({ mdwt: { otid, content } });
      }

      await onPostSave({
        otid: threadOtid,
        saveState: SaveState.Saved,
        kind: ChnotKind.ThreadV1,
      });

      lastSavedContentRef.current = currentBlocks;
      savedBlockOtidsRef.current = currentOtids;
    } finally {
      savingRef.current = false;
    }
  }, [threadOtid, onPostSave]);

  const handleContentChange = useCallback(
    (_content: string) => {
      if (saveTimerRef.current) clearTimeout(saveTimerRef.current);
      saveTimerRef.current = setTimeout(() => {
        handleBlockSave();
      }, 500);
    },
    [handleBlockSave],
  );

  useEffect(() => {
    return () => {
      if (saveTimerRef.current) clearTimeout(saveTimerRef.current);
    };
  }, []);

  const handleAddBlock = useCallback(() => {
    const view = cmRef.current?.view;
    if (!view) return;
    const newOtid = genTID();
    const insertText = `\n\n## [[${newOtid}]] `;
    const end = view.state.doc.length;
    view.dispatch({
      changes: { from: end, insert: insertText },
      selection: { anchor: end + insertText.length },
    });
    view.focus();
  }, []);

  return (
    <div className="flex flex-col w-full h-full min-h-0 items-center overflow-y-auto">
      {loading ? (
        <div />
      ) : (
        <div className="flex flex-col w-full items-center max-w-2xl px-4 py-6">
          <div className="flex w-full pb-4 mb-2 border-b border-border">
            <MdwtChnot
              otid={threadOtid}
              fullscreen={false}
              disableHeaderActions={true}
              onPostSave={handlePostSave}
              content={threadContent}
              placeholder="Thread Title"
            />
          </div>
          <div className="flex flex-col w-full">
            <MdwtEditor
              content={joinedDoc}
              foldGutter={false}
              fillParentHeight={false}
              onContentChange={handleContentChange}
              setCodeMirrorRef={(ref) => {
                cmRef.current = ref.current;
              }}
              placeholder="Add blocks with /tid on a heading line"
            />
            <div className="flex items-center gap-1.5 pt-3 pl-10">
              <button
                type="button"
                onClick={handleAddBlock}
                className="flex items-center gap-1 px-3 py-1.5 text-sm text-muted-foreground hover:text-primary hover:bg-accent rounded-lg transition-colors"
                title="Add new block"
              >
                <Plus className="w-4 h-4" />
                <span>Add Block</span>
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export const ChnotThreadMemo = React.memo(ChnotThread);

export default ChnotThread;
