import clsx from "clsx";
import { Copy, Download, Ellipsis } from "lucide-react";
import type React from "react";
import { useCallback, useMemo, useRef, useState } from "react";
import { Button } from "@/common/component/ui/button";
import { formatRelativeTime } from "@/lib/date-utils";

export const RecordButton = ({
  onClick,
  children,
}: {
  onClick: () => void;
  children: React.ReactNode;
}) => {
  return (
    <Button
      onClick={() => {
        onClick();
      }}
      className="p-1 rounded-full hover:bg-gray-200 focus:outline-none transition-colors"
      aria-label="show-full"
      tabIndex={0}
      variant={"ghost"}
    >
      {children}
    </Button>
  );
};

const RecordFrame = ({
  name,
  timestamp,
  limitHeight: initLimitHeight,
  onCopy,
  onDownload,
  logo,
  children,
  justifyEnd,
  buttons,
  viewMode,
  maxWidth,
  usage,
}: {
  name?: string;
  timestamp?: string;
  limitHeight?: boolean;
  justifyEnd?: boolean;
  onCopy?: () => void;
  onDownload?: () => void;
  logo?: React.ReactElement;
  children: React.ReactNode;
  buttons?: React.ReactNode;
  viewMode: boolean;
  maxWidth?: string;
  usage?: { inputTokens?: number; outputTokens?: number };
}) => {
  const contentRef = useRef<string>("");
  const handleCopy = useCallback(() => {
    if (onCopy) {
      onCopy();
    } else {
      navigator.clipboard.writeText(contentRef.current);
    }
  }, [onCopy]);

  const [limitHeight, setLimitHeight] = useState<boolean | undefined>(
    initLimitHeight,
  );

  const displayTime = useMemo(() => {
    if (!timestamp) return "Now";
    const date = new Date(timestamp);
    if (Number.isNaN(date.getTime())) return timestamp;
    return formatRelativeTime(date);
  }, [timestamp]);

  const fullTime = useMemo(() => {
    if (!timestamp) return undefined;
    const date = new Date(timestamp);
    if (Number.isNaN(date.getTime())) return timestamp;
    return date.toLocaleString();
  }, [timestamp]);

  return (
    <div
      className={clsx(
        "flex md:flex-row md:space-y-0 md:space-x-3 mx-4",
        justifyEnd && "justify-end ml-auto",
        maxWidth,
      )}
    >
      {logo && (
        <div className="flex-shrink-0 w-8 h-8 rounded-full bg-muted flex items-center justify-center overflow-hidden">
          {logo}
        </div>
      )}
      <div
        className={clsx(
          "flex-col min-w-0",
          justifyEnd ? "items-end" : "flex-1",
        )}
      >
        <div className="text-muted-foreground text-xs space-x-2 mb-1">
          {name && <span className="font-medium">{name}</span>}
          <span title={fullTime}>{displayTime}</span>
        </div>
        {limitHeight !== undefined && limitHeight ? (
          <div className={"max-h-160 overflow-hidden"}>{children}</div>
        ) : (
          children
        )}

        {viewMode || (
          <div className="space-x-2 flex">
            {buttons && buttons}
            {limitHeight !== undefined && (
              <RecordButton
                onClick={() => {
                  setLimitHeight((prev) => {
                    return !prev;
                  });
                }}
              >
                <Ellipsis className="h-4 w-4 text-gray-700" />
              </RecordButton>
            )}
            <RecordButton
              onClick={(): void => {
                handleCopy();
              }}
            >
              <Copy />
            </RecordButton>
            {onDownload && (
              <RecordButton onClick={onDownload}>
                <Download />
              </RecordButton>
            )}
            {usage && (
              <span className="text-muted-foreground/60 text-xs self-center">
                {usage.inputTokens ?? 0}→{usage.outputTokens ?? 0}
              </span>
            )}
          </div>
        )}
      </div>
    </div>
  );
};

export default RecordFrame;
