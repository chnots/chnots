import { ChevronLeft, ChevronRight } from "lucide-react";
import { Badge } from "@/common/component/ui/badge";
import { Button } from "@/common/component/ui/button";
import type { MdwtTagSearchType } from "@/krate/chnot/dto";
import type { TodoStateEnum } from "../toent-model";
import { TODO_STATE_LABEL, type ToentViewMode } from "./toent-page-shared";

export function ToentViewToolbar({
  viewMode,
  title,
  onPrev,
  onNext,
  onToday,
}: {
  viewMode: ToentViewMode;
  title: string;
  onPrev: () => void;
  onNext: () => void;
  onToday: () => void;
}) {
  if (viewMode === "list") {
    return null;
  }

  return (
    <div className="flex flex-wrap items-center justify-between gap-2 rounded-xl border bg-muted/40 p-2">
      <div className="flex items-center gap-2">
        <Button size="icon" variant="ghost" onClick={onPrev}>
          <ChevronLeft className="size-4" />
        </Button>
        <Button size="icon" variant="ghost" onClick={onNext}>
          <ChevronRight className="size-4" />
        </Button>
        <Button size="sm" variant="outline" onClick={onToday}>
          Today
        </Button>
      </div>
      <p className="text-sm font-medium text-muted-foreground">{title}</p>
    </div>
  );
}

export function ToentFilterSummaryBar({
  total,
  searchTerm,
  tags,
  enabledStates,
  stateCounts,
}: {
  total: number;
  searchTerm: string;
  tags?: MdwtTagSearchType;
  enabledStates: Record<TodoStateEnum, boolean>;
  stateCounts: Record<TodoStateEnum, number>;
}) {
  const selectedTags = tags?.id === "Inset" ? tags.data : [];
  const activeStates = Object.entries(enabledStates)
    .filter(([, enabled]) => enabled)
    .map(([state]) => state as TodoStateEnum);

  return (
    <div className="rounded-xl border bg-muted/30 p-2.5 text-xs">
      <div className="flex flex-wrap items-center gap-1.5">
        <Badge variant="outline">{total} items</Badge>
        {searchTerm.trim() ? (
          <Badge variant="outline">Search: {searchTerm.trim()}</Badge>
        ) : null}
        {selectedTags.map((tag) => (
          <Badge key={tag} variant="outline">
            #{tag}
          </Badge>
        ))}
        {activeStates.map((state) => (
          <Badge key={state} variant="secondary">
            {TODO_STATE_LABEL[state]} ({stateCounts[state]})
          </Badge>
        ))}
      </div>
    </div>
  );
}
