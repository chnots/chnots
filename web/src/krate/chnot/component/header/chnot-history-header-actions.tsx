import { Check, History, RotateCcw } from "lucide-react";
import { Button } from "@/common/component/ui/button";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/common/component/ui/popover";

type HistoryHeaderActionsProps<T> = {
  versions: T[];
  previewing: boolean;
  onOpenHistory: () => void;
  formatVersion: (version: T) => string;
  getVersionKey: (version: T) => number | string;
  onViewVersion: (version: T) => void | Promise<void>;
  onApply: () => void | Promise<void>;
  onLatest: () => void;
};

const HistoryHeaderActions = <T,>({
  versions,
  previewing,
  onOpenHistory,
  formatVersion,
  getVersionKey,
  onViewVersion,
  onApply,
  onLatest,
}: HistoryHeaderActionsProps<T>) => {
  const historyButton = (
    <Popover
      onOpenChange={(open) => {
        if (open) {
          onOpenHistory();
        }
      }}
    >
      <PopoverTrigger asChild>
        <Button variant="ghost" size="icon" title="History">
          <History className="w-4 h-4" />
        </Button>
      </PopoverTrigger>
      <PopoverContent align="end" className="w-72 p-2">
        <div className="max-h-80 overflow-auto space-y-1">
          {versions.length === 0 ? (
            <div className="text-sm text-muted-foreground px-2 py-1">
              No history
            </div>
          ) : (
            versions.map((version) => (
              <div
                key={getVersionKey(version)}
                className="flex items-center justify-between gap-2 px-2 py-1"
              >
                <span
                  className="text-xs text-muted-foreground"
                  title={String(getVersionKey(version))}
                >
                  {formatVersion(version)}
                </span>
                <Button
                  size="sm"
                  variant="outline"
                  onClick={() => void onViewVersion(version)}
                >
                  View
                </Button>
              </div>
            ))
          )}
        </div>
      </PopoverContent>
    </Popover>
  );

  return previewing ? (
    <>
      {historyButton}
      <Button
        variant="outline"
        size="icon"
        onClick={() => void onApply()}
        title="Apply history"
      >
        <Check className="w-4 h-4" />
      </Button>
      <Button variant="ghost" size="icon" onClick={onLatest} title="Latest">
        <RotateCcw className="w-4 h-4" />
      </Button>
    </>
  ) : (
    historyButton
  );
};

export default HistoryHeaderActions;
