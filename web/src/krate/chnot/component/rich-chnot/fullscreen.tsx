import { Fullscreen as FullscreenIcon } from "lucide-react";
import type { ReactNode } from "react";
import { Button } from "@/common/component/ui/button";

const Fullscreen = ({
  children,
  onSetFullscreen,
}: {
  children: ReactNode;
  onSetFullscreen?: (flag: boolean) => void;
}) => {
  return (
    <div className="w-screen h-screen z-50 flex flex-col fixed bottom-0 left-0 m-0 p-2 bg-background items-center">
      {onSetFullscreen && (
        <Button
          onClick={() => onSetFullscreen(false)}
          className="absolute top-1 right-1 z-49"
        >
          <FullscreenIcon />
        </Button>
      )}
      {children}
    </div>
  );
};

export default Fullscreen;
