import { History } from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import { Button } from "@/common/component/ui/button";
import { SaveState } from "@/common/types";
import { useIsMobile } from "@/hooks/use-mobile";
import {
  getChainToNode,
  HistoryMobileSheet,
  HistoryTreePanel,
} from "@/krate/llmchat/component/history-tree";
import SessionContainer, {
  type LLMChatContextProps,
  LLMChatEditorProvider,
} from "@/krate/llmchat/component/session";
import {
  llmchatSessionRecordFetch,
  llmchatSessionRecordFetchAll,
} from "@/krate/llmchat/service";
import { useLLMChatStore } from "@/krate/llmchat/store";
import type { LLMChatRecordVO } from "@/krate/llmchat/vo";
import type { TID } from "@/lib/id_util";
import { ChnotKind } from "../../po";
import { chnotHeadStore } from "../../store";
import Fullscreen from "./fullscreen";
import type { RichPropProps } from "./types";

const LLMChatChnot = ({
  otid,
  fullscreen,
  readonly,
  onPostSave,
  onSetFullscreen,
  disableHeaderActions,
}: RichPropProps) => {
  const persistedIds = useRef<Set<TID>>(new Set());
  const isMobile = useIsMobile();

  const [props, setProps] = useState<LLMChatContextProps | undefined>(
    undefined,
  );
  const [historyOpen, setHistoryOpen] = useState(false);
  const [historyRecords, setHistoryRecords] = useState<LLMChatRecordVO[]>([]);
  const { refreshTemplates, refreshBots } = useLLMChatStore();

  useEffect(() => {
    refreshTemplates();
    refreshBots();
  }, [refreshTemplates, refreshBots]);

  useEffect(() => {
    (async () => {
      persistedIds.current = new Set();

      if (otid) {
        const { session, records } = await llmchatSessionRecordFetch({
          session_otid: otid,
        });
        if (session) {
          setProps(() => {
            persistedIds.current?.add(session.otid);
            records.forEach((r) => {
              persistedIds.current?.add(r.otid);
            });
            return {
              sessionOtid: otid,
              records: records,
              session: session,
              persistedIds: persistedIds,
            };
          });
        } else {
          setProps({
            sessionOtid: otid,
            persistedIds: persistedIds,
          });
        }
      }
    })();
  }, [otid]);

  const handleToggleHistory = useCallback(async () => {
    const next = !historyOpen;
    setHistoryOpen(next);
    if (next && otid) {
      const records = await llmchatSessionRecordFetchAll(otid);
      setHistoryRecords(records);
    }
  }, [historyOpen, otid]);

  useEffect(() => {
    if (historyOpen && otid) {
      llmchatSessionRecordFetchAll(otid).then(setHistoryRecords);
    }
  }, [historyOpen, otid, props?.records]);

  const handleSelectNode = useCallback(
    (nodeOtid: TID) => {
      const chain = getChainToNode(historyRecords, nodeOtid);
      for (const r of chain) {
        persistedIds.current?.add(r.otid);
      }
      setProps((prev) => {
        if (!prev) return prev;
        return { ...prev, records: chain };
      });
    },
    [historyRecords],
  );

  useEffect(() => {
    if (disableHeaderActions || !otid || readonly) {
      return;
    }

    const key = `llmchat-history-${otid}`;
    const actions = (
      <Button
        variant={historyOpen ? "secondary" : "ghost"}
        size="icon"
        title="History"
        onClick={() => void handleToggleHistory()}
      >
        <History className="w-4 h-4" />
      </Button>
    );

    chnotHeadStore.getState().registerHeaderActions(key, actions);
    return () => {
      chnotHeadStore.getState().unregisterHeaderActions(key);
    };
  }, [disableHeaderActions, otid, readonly, historyOpen, handleToggleHistory]);

  if (!props) return null;

  const chatContent = (
    <SessionContainer
      readonly={readonly ?? false}
      onPostSave={
        readonly
          ? undefined
          : async (_session, title) => {
              await onPostSave({
                otid,
                saveState: SaveState.Saved,
                kind: ChnotKind.LLMChat,
                title,
              });
            }
      }
    />
  );

  const historySidebar = historyOpen && !isMobile && !readonly && (
    <div className="w-72 border-l flex flex-col h-full shrink-0">
      <div className="px-3 py-2 border-b text-sm font-medium text-muted-foreground">
        Chat History
      </div>
      <HistoryTreePanel records={historyRecords} onSelect={handleSelectNode} />
    </div>
  );

  const mobileSheet = isMobile && (
    <HistoryMobileSheet
      open={historyOpen}
      onOpenChange={setHistoryOpen}
      records={historyRecords}
      onSelect={handleSelectNode}
    />
  );

  return (
    <LLMChatEditorProvider props={props}>
      {fullscreen ? (
        <Fullscreen onSetFullscreen={onSetFullscreen}>
          <div className="flex h-full w-full">
            <div className="flex-1 min-w-0">{chatContent}</div>
            {historySidebar}
          </div>
        </Fullscreen>
      ) : (
        <div className="flex w-full h-full">
          <div className="flex-1 min-w-0">
            {readonly ? (
              <SessionContainer readonly={true} />
            ) : (
              <SessionContainer
                readonly={false}
                onPostSave={async (_session, title) => {
                  await onPostSave({
                    otid,
                    saveState: SaveState.Saved,
                    kind: ChnotKind.LLMChat,
                    title,
                  });
                }}
              />
            )}
          </div>
          {historySidebar}
          {mobileSheet}
        </div>
      )}
    </LLMChatEditorProvider>
  );
};

export default LLMChatChnot;
