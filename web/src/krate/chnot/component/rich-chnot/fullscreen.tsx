import { Fullscreen as FullscreenIcon } from "lucide-react";
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
    <div className="w-screen h-screen z-50 flex flex-col fixed bottom-0 left-0 m-0 p-0 bg-background">
      <div className="w-full flex items-center justify-end gap-1 border-b px-2 py-1">
        {headerActions.map((ha) => (
          <div key={ha.key}>{ha.actions}</div>
        ))}
        {onSetFullscreen && (
          <Button
            variant="ghost"
            size="icon"
            onClick={() => onSetFullscreen(false)}
          >
            <FullscreenIcon />
          </Button>
        )}
      </div>
      <div className="flex-1 min-h-0 w-full overflow-auto p-2">{children}</div>
    </div>
  );
};

export default Fullscreen;
