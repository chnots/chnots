import dayjs from "dayjs";
import { MessageSquare, User } from "lucide-react";
import { useMemo } from "react";
import KSVG from "@/common/component/svg";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
} from "@/common/component/ui/sheet";
import { getBlockContent } from "@/krate/llmchat/po";
import { useLLMChatStore } from "@/krate/llmchat/store";
import type { LLMChatRecordVO } from "@/krate/llmchat/vo";
import { tidToDate } from "@/lib/date-utils";
import type { TID } from "@/lib/id_util";

type HistoryTreeNode = {
  record: LLMChatRecordVO;
  children: HistoryTreeNode[];
};

type HistoryTreeRoot = {
  roots: HistoryTreeNode[];
  allRecords: LLMChatRecordVO[];
};

export function buildHistoryTree(records: LLMChatRecordVO[]): HistoryTreeRoot {
  const seen = new Set<TID>();
  const deduped = records.filter((r) => {
    if (seen.has(r.otid)) return false;
    seen.add(r.otid);
    return true;
  });

  const childrenMap = new Map<TID | null, LLMChatRecordVO[]>();

  for (const r of deduped) {
    const key = r.pre_record_otid ?? null;
    const list = childrenMap.get(key) ?? [];
    list.push(r);
    childrenMap.set(key, list);
  }

  function buildNode(record: LLMChatRecordVO): HistoryTreeNode {
    const kids = childrenMap.get(record.otid) ?? [];
    return {
      record,
      children: kids.map(buildNode),
    };
  }

  const rootRecords = childrenMap.get(null) ?? [];
  const roots = rootRecords.map(buildNode);

  return { roots, allRecords: records };
}

export function getChainToNode(
  allRecords: LLMChatRecordVO[],
  targetOtid: TID,
): LLMChatRecordVO[] {
  const byOtid = new Map<TID, LLMChatRecordVO>();
  for (const r of allRecords) {
    byOtid.set(r.otid, r);
  }

  const chain: LLMChatRecordVO[] = [];
  let current: TID | null | undefined = targetOtid;
  while (current) {
    const record = byOtid.get(current);
    if (!record) break;
    chain.push(record);
    current = record.pre_record_otid;
  }

  chain.reverse();
  return chain;
}

function RecordRoleIcon({ record }: { record: LLMChatRecordVO }) {
  const { bots, templates } = useLLMChatStore();

  if (record.role === "user") {
    return <User className="w-3.5 h-3.5 shrink-0" />;
  }

  const bot = record.role_id ? bots.get(record.role_id) : undefined;
  if (bot?.svg_logo) {
    return (
      <KSVG src={bot.svg_logo} className="w-3.5 h-3.5 shrink-0 rounded-sm" />
    );
  }

  const tmpl = record.role_id ? templates.get(record.role_id) : undefined;
  if (tmpl?.svg_logo) {
    return (
      <KSVG src={tmpl.svg_logo} className="w-3.5 h-3.5 shrink-0 rounded-sm" />
    );
  }

  return (
    <div className="w-3.5 h-3.5 shrink-0 rounded-full bg-muted flex items-center justify-center">
      <span className="text-[8px] leading-none text-muted-foreground">
        {record.role === "system" ? "S" : "A"}
      </span>
    </div>
  );
}

function formatRecordTime(otid: TID): string {
  const date = tidToDate(otid);
  return date ? dayjs(date).format("MM-DD HH:mm") : "";
}

function getRecordSummary(record: LLMChatRecordVO): string {
  const body = getBlockContent(record.content, "content");
  const normalized = body.replaceAll(/\s+/g, " ").trim();
  return normalized.slice(0, 60) || record.role;
}

function TreeNode({
  node,
  depth,
  onSelect,
}: {
  node: HistoryTreeNode;
  depth: number;
  onSelect: (otid: TID) => void;
}) {
  const isUser = node.record.role === "user";
  const isUserOrAssistant =
    node.record.role === "user" || node.record.role === "assistant";
  const isLeaf = node.children.length === 0;

  return (
    <div>
      <button
        type="button"
        className={`flex items-center gap-1.5 w-full text-left px-2 py-1.5 rounded-sm text-xs transition-colors ${
          isUser ? "cursor-default" : "hover:bg-accent cursor-pointer"
        }`}
        style={{ paddingLeft: `${depth * 8 + 8}px` }}
        onClick={() => {
          if (!isUser) onSelect(node.record.otid);
        }}
      >
        <RecordRoleIcon record={node.record} />
        {isUserOrAssistant && (
          <span
            className={`truncate flex-1 min-w-0 ${isUser ? "text-muted-foreground" : ""}`}
          >
            {getRecordSummary(node.record)}
          </span>
        )}
        {isLeaf && !isUser && (
          <MessageSquare className="w-3 h-3 shrink-0 text-muted-foreground/50" />
        )}
      </button>
      {node.children.map((child) => (
        <TreeNode
          key={child.record.otid}
          node={child}
          depth={depth + 1}
          onSelect={onSelect}
        />
      ))}
    </div>
  );
}

export function HistoryTreePanel({
  records,
  onSelect,
}: {
  records: LLMChatRecordVO[];
  onSelect: (otid: TID) => void;
}) {
  const tree = useMemo(() => buildHistoryTree(records), [records]);

  if (tree.roots.length === 0) {
    return (
      <div className="text-sm text-muted-foreground px-2 py-3">No records</div>
    );
  }

  return (
    <div className="overflow-auto h-full">
      {tree.roots.map((root) => (
        <TreeNode
          key={root.record.otid}
          node={root}
          depth={0}
          onSelect={onSelect}
        />
      ))}
    </div>
  );
}

export function HistoryMobileSheet({
  open,
  onOpenChange,
  records,
  onSelect,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  records: LLMChatRecordVO[];
  onSelect: (otid: TID) => void;
}) {
  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent side="bottom" className="max-h-[70vh]">
        <SheetHeader>
          <SheetTitle>Chat History</SheetTitle>
        </SheetHeader>
        <div className="px-4 pb-4 overflow-auto">
          <HistoryTreePanel records={records} onSelect={onSelect} />
        </div>
      </SheetContent>
    </Sheet>
  );
}
