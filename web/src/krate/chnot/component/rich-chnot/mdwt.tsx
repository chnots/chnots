import {
  normalizeBlockContent,
} from "@chnots/md-codemirror";
import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import dayjs from "dayjs";
import { useCallback, useEffect, useRef, useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { SaveState } from "@/common/types";
import useDebounce from "@/hooks/use-debounce";
import { useKSpaceStore } from "@/krate/kspace/store";
import { MdwtEditorMemo } from "@/krate/mdwt/component/mdwt-editor";
import type { MdwtCommitReq } from "@/krate/mdwt/dto";
import {
  mdwtCommit,
  mdwtContentLoad,
  mdwtHistoryApply,
  mdwtHistoryFetch,
  mdwtHistoryList,
} from "@/krate/mdwt/service";
import { tidToDate } from "@/lib/date-utils";
import { ChnotKind } from "../../po";
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

  const cmRef = useRef<ReactCodeMirrorRef>(null);

  const { kspace } = useKSpaceStore((s) => ({ kspace: s.currentKSpace }));

  useEffect(() => {
    if (initialContent === undefined) {
      (async () => {
        try {
          const rsp = await mdwtContentLoad({ otid });
          const c = rsp.content;
          setContent(c);
          cachedContentRef.current = c;
          if (onContentChange && c) onContentChange(c);
        } catch {
          setContent("");
          cachedContentRef.current = "";
        }
      })();
    }
  }, [initialContent, otid]);

  const directlySave = async () => {
    if (!toSaveArg.current) {
      return;
    }

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
          kspace: kspace,
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
