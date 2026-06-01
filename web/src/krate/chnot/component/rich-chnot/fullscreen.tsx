import { Minimize2 } from "lucide-react";
import type { ReactNode } from "react";
import { Button } from "@/common/component/ui/button";
import { useChnotStore } from "../../store";

const Fullscreen = ({
  children,
  onSetFullscreen,
}: {
  children: ReactNode;
  onSetFullscreen?: (flag: boolean) => void;
}) => {
  const { headerActions } = useChnotStore((s) => {
    return {
      headerActions: s.headerActions,
    };
  });

  return (
    <div
      className="inset-0 z-50 flex flex-col fixed bg-background"
      role="dialog"
      aria-modal="true"
    >
      <div className="flex items-center justify-end gap-1 border-b px-2 py-1 shrink-0">
        {headerActions.map((ha) => (
          <div key={ha.key}>{ha.actions}</div>
        ))}
        {onSetFullscreen && (
          <Button
            variant="ghost"
            size="icon"
            onClick={() => onSetFullscreen(false)}
            title="Exit fullscreen"
          >
            <Minimize2 className="size-4" />
          </Button>
        )}
      </div>
      <div className="flex-1 min-h-0 overflow-auto p-2">{children}</div>
    </div>
  );
};

export default Fullscreen;
