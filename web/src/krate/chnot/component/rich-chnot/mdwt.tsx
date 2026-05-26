import type { ReactCodeMirrorRef } from "@uiw/react-codemirror";
import dayjs from "dayjs";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { SaveState } from "@/common/types";
import useDebounce from "@/hooks/use-debounce";
import { useKSpaceStore } from "@/krate/kspace/store";
import { MdwtEditorMemo } from "@/krate/mdwt/component/mdwt-editor";
import {
  dispatchThreadData,
  threadWidgetExtension,
  type ThreadWidgetItem,
} from "@/krate/mdwt/component/thread-widget-extension";
import ThreadEditorPanel from "@/krate/mdwt/component/thread-editor-sheet";
import type { MdwtCommitReq } from "@/krate/mdwt/dto";
import {
  mdwtCommit,
  mdwtContentLoad,
  mdwtHistoryApply,
  mdwtHistoryFetch,
  mdwtHistoryList,
  mdwtRecordList,
} from "@/krate/mdwt/service";
import { fetchExcalidraw } from "@/krate/graph/excalidraw/service";
import { fetchMindExilir } from "@/krate/graph/mind-elixir/service";
import { tidToDate } from "@/lib/date-utils";
import { chnotMetaCommit, chnotThreadMetaFetch } from "../../service";
import { ChnotKind } from "../../po";
import { chnotHeadStore } from "../../store";
import HistoryHeaderActions from "../header/chnot-history-header-actions";
import type { RichPropProps } from "./types";
import type { HeadingCompletionConfig } from "@/krate/mdwt/codemirror/mdwt/heading-chnot-completion";

const MdwtChnot = ({
  otid,
  readonly,
  placeholder,
  onPostSave,
  onContentChange,
  content: initialContent,
  disableHeaderActions,
}: RichPropProps & {
  placeholder?: string;
  content?: string;
  onContentChange?: (content: string) => void;
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

  const cmRefObj = useRef<React.RefObject<ReactCodeMirrorRef | null> | null>(null);

  const { kspace } = useKSpaceStore((s) => ({ kspace: s.currentKSpace }));

  // Thread widget state
  const [selectedItem, setSelectedItem] = useState<
    { otid: number; kind: ChnotKind } | undefined
  >();
  const threadItemsRef = useRef<ThreadWidgetItem[]>([]);
  const threadExtension = useMemo(() => threadWidgetExtension(), []);
  const threadLoadTriggered = useRef(false);

  const handleThreadItemClick = useCallback(
    (otid: number, kind: ChnotKind) => {
      setSelectedItem({ otid, kind });
    },
    [],
  );

  const loadThreadData = useCallback(
    async (view: import("@codemirror/view").EditorView) => {
      try {
        const rsp = await chnotThreadMetaFetch({ otid });
        if (!rsp.chnot_meta_sorted.length) return;

        const childOtids = rsp.chnot_meta_sorted.map((m) => m.meta.otid);
        const mdwtRsp = await mdwtRecordList({ mdwt_otids: childOtids });

        const items: ThreadWidgetItem[] = [];

        for (const threadMeta of rsp.chnot_meta_sorted) {
          const childOtid = threadMeta.meta.otid;
          const record = mdwtRsp.mdwt_map[childOtid];
          const content = record?.content ?? "";
          const lines = content.split("\n");
          const titleLine = lines[0] ?? "";
          const mdwtContent = lines.slice(1).join("\n");

          const item: ThreadWidgetItem = {
            otid: childOtid,
            kind: threadMeta.meta.kind,
            headingLevel: threadMeta.heading_level ?? 2,
            titleLine,
            mdwtContent,
          };

          if (threadMeta.meta.kind === ChnotKind.ExcalidrawV1) {
            try {
              const excState = await fetchExcalidraw(childOtid, new Map());
              if (excState) item.kindData = excState;
            } catch {}
          } else if (threadMeta.meta.kind === ChnotKind.MindMapV1) {
            try {
              const mindData = await fetchMindExilir(childOtid);
              if (mindData) item.kindData = mindData;
            } catch {}
          }

          items.push(item);
        }

        threadItemsRef.current = items;
        dispatchThreadData(view, {
          items,
          onItemClick: handleThreadItemClick,
        });
      } catch {
        // Thread data not available
      }
    },
    [otid, handleThreadItemClick],
  );

  const triggerLoadThreadData = useCallback(
    (view: import("@codemirror/view").EditorView) => {
      if (threadLoadTriggered.current) return;
      threadLoadTriggered.current = true;
      void loadThreadData(view);
    },
    [loadThreadData],
  );

  // Load thread data when CM view is ready
  useEffect(() => {
    if (readonly) return;
    let count = 0;
    const interval = setInterval(() => {
      count++;
      const view = cmRefObj.current?.current?.view;
      if (view) {
        clearInterval(interval);
        triggerLoadThreadData(view);
      } else if (count > 20) {
        clearInterval(interval);
      }
    }, 100);
    return () => clearInterval(interval);
  }, [readonly, content, triggerLoadThreadData]);

  const refreshThreadWidgets = useCallback(async () => {
    const view = cmRefObj.current?.current?.view;
    if (!view) return;
    await loadThreadData(view);
  }, [loadThreadData]);

  const getExcludeOtids = useCallback((): number[] => {
    const content = cachedContentRef.current ?? "";
    const otids: number[] = [];
    for (const m of content.matchAll(/\[\[(\d{13,16})\]\]/g)) {
      otids.push(Number(m[1]));
    }
    return otids;
  }, []);

  const handleCreateChnot = useCallback(
    async (childOtid: number, kind: ChnotKind) => {
      await chnotMetaCommit({
        metas: [{ otid: childOtid, kind, kspace }],
      });
      if (kind !== ChnotKind.MDWT) {
        setSelectedItem({ otid: childOtid, kind });
      }
    },
    [kspace],
  );

  const handleRefExisting = useCallback(
    async (childOtid: number, kind: ChnotKind) => {
      const view = cmRefObj.current?.current?.view;
      if (!view) return;

      try {
        const mdwtRsp = await mdwtRecordList({ mdwt_otids: [childOtid] });
        const record = mdwtRsp.mdwt_map[childOtid];
        const rawContent = record?.content ?? "";
        const lines = rawContent.split("\n");
        const titleLine = lines[0] ?? "";
        const mdwtContent = lines.slice(1).join("\n");

        const item: ThreadWidgetItem = {
          otid: childOtid,
          kind,
          headingLevel: 2,
          titleLine,
          mdwtContent,
        };

        if (kind === ChnotKind.ExcalidrawV1) {
          try {
            const excState = await fetchExcalidraw(childOtid, new Map());
            if (excState) item.kindData = excState;
          } catch {}
        } else if (kind === ChnotKind.MindMapV1) {
          try {
            const mindData = await fetchMindExilir(childOtid);
            if (mindData) item.kindData = mindData;
          } catch {}
        }

        const items = [...threadItemsRef.current, item];
        threadItemsRef.current = items;
        dispatchThreadData(view, {
          items,
          onItemClick: handleThreadItemClick,
        });
      } catch {
        // failed to load preview data
      }
    },
    [handleThreadItemClick],
  );

  const headingCompletionConfig = useMemo<
    HeadingCompletionConfig | undefined
  >(() => {
    if (readonly) return undefined;
    return {
      parentOtid: otid,
      getExcludeOtids,
      onCreateChnot: handleCreateChnot,
      onRefExisting: handleRefExisting,
    };
  }, [readonly, otid, getExcludeOtids, handleCreateChnot, handleRefExisting]);

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

  const noop = useCallback(() => {}, []);

  const renderEditor = (editorContent: string | undefined, isReadonly: boolean) => (
    <MdwtEditorMemo
      content={editorContent}
      onContentChange={noop}
      foldGutter={false}
      readonly={isReadonly}
      extraExtensions={[threadExtension]}
      headingCompletionConfig={headingCompletionConfig}
    />
  );

  return previewMode ? (
    renderEditor(previewContent, true)
  ) : readonly ? (
    renderEditor(cachedContentRef.current ?? "", true)
  ) : (
    content !== undefined && (
      <div
        className="flex w-full h-full min-h-0 break-all"
        onBlur={() => directlySave()}
        role="none"
      >
        <div
          className="flex-1 min-w-0"
        >
          <MdwtEditorMemo
            placeholder={placeholder}
            content={content}
            onContentChange={handleContentChange}
            foldGutter={false}
            extraExtensions={[threadExtension]}
            headingCompletionConfig={headingCompletionConfig}
            setCodeMirrorRef={(ref) => {
              cmRefObj.current = ref;
            }}
          />
        </div>
        {selectedItem && (
          <ThreadEditorPanel
            item={selectedItem}
            onClose={() => {
              setSelectedItem(undefined);
              void refreshThreadWidgets();
            }}
            onSaved={() => {}}
          />
        )}
      </div>
    )
  );
};

export default MdwtChnot;
