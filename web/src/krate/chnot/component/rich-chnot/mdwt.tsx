import {
  normalizeBlockContent,
  splitDocumentByBlocks,
} from "@chnots/md-codemirror";
import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import dayjs from "dayjs";
import { useCallback, useEffect, useRef, useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { SaveState } from "@/common/types";
import useDebounce from "@/hooks/use-debounce";
import { MdwtEditorMemo } from "@/krate/mdwt/component/mdwt-editor";
import type { MdwtCommitReq } from "@/krate/mdwt/dto";
import {
  mdwtCommit,
  mdwtHistoryApply,
  mdwtHistoryFetch,
  mdwtHistoryList,
  mdwtRecordList,
} from "@/krate/mdwt/service";
import { arraysAreEqual } from "@/lib/col-util";
import { tidToDate } from "@/lib/date-utils";
import type { TID } from "@/lib/id_util";
import { ChnotKind } from "../../po";
import {
  chnotThreadMetaFetch,
  chnotThreadOrderArchive,
  chnotThreadOrderCommit,
} from "../../service";
import { chnotHeadStore } from "../../store";
import HistoryHeaderActions from "../header/chnot-history-header-actions";
import type { RichPropProps } from "./types";

const MarkdownViewer = ({
  content: initialContent,
  keepBreak,
}: {
  content: string;
  keepBreak?: boolean;
}) => {
  const content = keepBreak
    ? initialContent.replaceAll("\n", "  \n")
    : initialContent;
  return (
    <div
      className={
        "prose prose-sm max-w-none prose-code:text-wrap prose-code:break-all prose-code:!p-2 min-w-full break-all h-full"
      }
    >
      <ReactMarkdown remarkPlugins={[remarkGfm]}>{content}</ReactMarkdown>
    </div>
  );
};

function joinMdwtBlocks(
  blocks: Array<{ otid: number; content: string }>,
): string {
  return blocks
    .map((b) => normalizeBlockContent(b.otid, b.content))
    .join("\n\n");
}

/**
 *
 * @param content if content is undefined, try to fetch mdwt, or just use it.
 * @returns
 */
const MdwtChnot = ({
  otid,
  readonly,
  placeholder,
  onPostSave,
  onContentChange,
  content: initialContent,
  fillParentHeight,
  disableHeaderActions,
}: RichPropProps & {
  placeholder?: string;
  content?: string;
  onContentChange?: (content: string) => void;
  fillParentHeight?: boolean;
}) => {
  const formatTid = useCallback((tid: number) => {
    const date = tidToDate(tid);
    return date ? dayjs(date).format("YYMM-DD HH:mm:ss") : String(tid);
  }, []);

  // use RefObject to avoid rerender
  const cachedContentRef = useRef<string | undefined>(initialContent);
  const toSaveArg = useRef<MdwtCommitReq>(null);
  const [content, setContent] = useState<string | undefined>(initialContent);
  const [historyVersions, setHistoryVersions] = useState<number[]>([]);
  const [previewTid, setPreviewTid] = useState<number | undefined>(undefined);
  const [previewContent, setPreviewContent] = useState<string>("");
  const previewMode = previewTid !== undefined;

  // Block tracking refs for thread-based save/load
  const cmRef = useRef<ReactCodeMirrorRef>(null);
  const savedBlockOtidsRef = useRef<TID[]>([]);
  const lastSavedContentRef = useRef<Map<number, string>>(new Map());
  const savingRef = useRef(false);

  useEffect(() => {
    if (initialContent === undefined) {
      (async () => {
        try {
          const threadRsp = await chnotThreadMetaFetch({ otid });
          const childOtids = threadRsp.chnot_meta_sorted.map(
            (cm) => cm.meta.otid,
          );

          if (childOtids.length > 0) {
            const mdwtRsp = await mdwtRecordList({
              mdwt_otids: [...childOtids],
            });
            const blocks = childOtids.map((otid) => ({
              otid,
              content: mdwtRsp.mdwt_map[otid]?.content ?? "",
            }));
            const joined = joinMdwtBlocks(blocks);
            setContent(joined);
            cachedContentRef.current = joined;
            if (onContentChange) onContentChange(joined);

            const savedMap = new Map<number, string>();
            for (const b of blocks) {
              savedMap.set(b.otid, normalizeBlockContent(b.otid, b.content));
            }
            lastSavedContentRef.current = savedMap;
            savedBlockOtidsRef.current = childOtids;
          } else {
            const rsp = await mdwtRecordList({ mdwt_otids: [otid] });
            const mdwt = rsp.mdwt_map[otid];
            const c = mdwt?.content ?? "";
            setContent(c);
            cachedContentRef.current = c;
            if (onContentChange && c) onContentChange(c);
          }
        } catch {
          const rsp = await mdwtRecordList({ mdwt_otids: [otid] });
          const mdwt = rsp.mdwt_map[otid];
          const c = mdwt?.content ?? "";
          setContent(c);
          cachedContentRef.current = c;
          if (onContentChange && c) onContentChange(c);
        }
      })();
    }
  }, [initialContent, otid]);

  const directlySave = async () => {
    const view = cmRef.current?.view;

    // Try block-based save via thread services
    if (view && !savingRef.current) {
      const currentBlocks = splitDocumentByBlocks(view.state);

      if (currentBlocks.size > 0) {
        savingRef.current = true;
        try {
          const previousBlocks = lastSavedContentRef.current;
          const previousOtids = savedBlockOtidsRef.current;
          const currentOtids = [...currentBlocks.keys()];

          const added = currentOtids.filter(
            (otid) => !previousBlocks.has(otid),
          );
          const changed = currentOtids.filter(
            (otid) =>
              previousBlocks.has(otid) &&
              previousBlocks.get(otid) !== currentBlocks.get(otid),
          );
          const deleted = previousOtids.filter(
            (otid) => !currentBlocks.has(otid),
          );

          if (
            added.length > 0 ||
            deleted.length > 0 ||
            !arraysAreEqual(currentOtids, previousOtids, (a, b) => a === b)
          ) {
            await chnotThreadOrderCommit({
              thread_otid: otid,
              orders: currentOtids.map((o) => ({ otid: o, closed: false })),
            });
          }

          if (deleted.length > 0) {
            await chnotThreadOrderArchive({
              thread_otid: otid,
              otids: deleted,
            });
          }

          const dirty = [...added, ...changed]
            .map((otid) => ({
              otid,
              content: currentBlocks.get(otid)!,
            }))
            .filter((b) => b.content !== undefined);

          await Promise.all(
            dirty.map((b) =>
              mdwtCommit({ mdwt: { otid: b.otid, content: b.content } }),
            ),
          );

          lastSavedContentRef.current = currentBlocks;
          savedBlockOtidsRef.current = currentOtids;

          onPostSave({
            otid,
            saveState: SaveState.Saved,
            kind: ChnotKind.MDWT,
          });

          toSaveArg.current = null;
          return;
        } finally {
          savingRef.current = false;
        }
      }
    }

    // Fallback: single mdwt save
    if (toSaveArg.current) {
      try {
        onPostSave({
          otid,
          saveState: SaveState.Saving,
          kind: ChnotKind.MDWT,
        });
        const rsp = await mdwtCommit(toSaveArg.current);
        onPostSave({
          otid,
          saveState: SaveState.Saved,
          title: rsp.title,
          kind: ChnotKind.MDWT,
        });
        toSaveArg.current = null;
      } catch (_ex) {
        onPostSave({
          otid,
          saveState: SaveState.Error,
          title: "<unable to save>",
          kind: ChnotKind.MDWT,
        });
      }
    }
  };

  const debounceSave = useDebounce(
    async () => {
      directlySave();
    },
    {
      duration: 2000,
      executeOnUnmount: true,
    },
  );
  const handleContentChange = useCallback(
    (content: string) => {
      if (content !== cachedContentRef.current) {
        if (onContentChange) {
          onContentChange(content);
        }
        cachedContentRef.current = content;
      }
      const req: MdwtCommitReq = {
        mdwt: {
          otid: otid,
          content: content,
        },
      };
      toSaveArg.current = req;
      debounceSave();
    },
    [debounceSave, otid],
  );

  const loadHistoryList = useCallback(async () => {
    const rsp = await mdwtHistoryList({ otid });
    setHistoryVersions(rsp.versions.map((v) => v.tid));
  }, [otid]);

  const viewHistory = useCallback(
    async (tid: number) => {
      const rsp = await mdwtHistoryFetch({ otid, tid });
      setPreviewTid(tid);
      setPreviewContent(rsp.content ?? "");
    },
    [otid],
  );

  const leavePreview = useCallback(() => {
    setPreviewTid(undefined);
    setPreviewContent("");
  }, []);

  const applyHistory = useCallback(async () => {
    if (!previewTid) {
      return;
    }
    const rsp = await mdwtHistoryApply({
      otid,
      tid: previewTid,
    });
    const next = rsp.content ?? previewContent;
    cachedContentRef.current = next;
    setContent(next);
    onContentChange?.(next);
    leavePreview();
    onPostSave({
      otid,
      saveState: SaveState.Saved,
      title: next.split("\n").at(0) ?? "",
      kind: ChnotKind.MDWT,
    });
  }, [
    otid,
    previewTid,
    previewContent,
    onPostSave,
    onContentChange,
    leavePreview,
  ]);

  useEffect(() => {
    if (disableHeaderActions) {
      return;
    }
    const key = `mdwt-history-${otid}`;
    const actions = (
      <HistoryHeaderActions
        versions={historyVersions}
        previewing={previewMode}
        onOpenHistory={() => {
          void loadHistoryList();
        }}
        formatVersion={(tid) => formatTid(tid)}
        getVersionKey={(tid) => tid}
        onViewVersion={(tid) => viewHistory(tid)}
        onApply={applyHistory}
        onLatest={leavePreview}
      />
    );

    chnotHeadStore.getState().registerHeaderActions(key, actions);
    return () => {
      chnotHeadStore.getState().unregisterHeaderActions(key);
    };
  }, [
    disableHeaderActions,
    otid,
    previewMode,
    historyVersions,
    formatTid,
    loadHistoryList,
    viewHistory,
    applyHistory,
    leavePreview,
  ]);

  return previewMode ? (
    <MarkdownViewer content={previewContent} keepBreak={true} />
  ) : readonly ? (
    <MarkdownViewer content={cachedContentRef.current ?? ""} keepBreak={true} />
  ) : (
    content !== undefined && (
      <div
        className="flex flex-col w-full h-full min-h-0 break-all"
        onBlur={() => directlySave()}
        role="none"
      >
        <div
          className={
            fillParentHeight
              ? "flex-1 min-h-0 h-full overflow-hidden"
              : undefined
          }
        >
          <MdwtEditorMemo
            placeholder={placeholder}
            content={content}
            onContentChange={handleContentChange}
            foldGutter={false}
            fillParentHeight={fillParentHeight}
            setCodeMirrorRef={(ref) => {
              cmRef.current = ref.current;
            }}
          />
        </div>
      </div>
    )
  );
};

export default MdwtChnot;
