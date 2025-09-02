import clsx from "clsx";
import Icon from "@/common/component/icon";
import React, { useCallback, useRef, useState } from "react";
import { Button as KButton } from "@/common/component/ui/button";

const RecordFrame = ({
  name,
  timestamp,
  limitHeight: initLimitHeight,
  onRegenerate,
  onCopy,
  onEdit,
  onOk,
  logo,
  children,
  justifyEnd,
}: {
  name?: string;
  timestamp?: string;
  limitHeight?: boolean;
  justifyEnd?: boolean;
  onCopy?: () => void;
  onRegenerate?: () => void;
  onEdit?: () => void;
  onOk?: () => void;
  logo?: React.ReactElement;
  children: React.ReactNode;
}) => {
  const contentRef = useRef<string>("");
  const handleCopy = useCallback(() => {
    if (onCopy) {
      onCopy();
    } else {
      navigator.clipboard.writeText(contentRef.current);
    }
  }, []);

  const [limitHeight, setLimitHeight] = useState<boolean | undefined>(
    initLimitHeight,
  );

  return (
    <div
      className={clsx(
        "flex md:flex-row md:space-y-0 md:space-x-4 mx-4 my-8",
        justifyEnd && "justify-end",
      )}
    >
      {logo && <>{logo}</>}
      <div className={clsx("flex-col")}>
        <div className="text-gray-500 text-xs space-x-2">
          <span>{name}</span>
          <span>{timestamp ?? "Now"}</span>
        </div>
        {limitHeight != undefined && limitHeight ? (
          <div className={"max-h-160 overflow-hidden"}>{children}</div>
        ) : (
          <>{children}</>
        )}

        <div className="space-x-2 mt-1 flex">
          {onRegenerate && (
            <button
              onClick={onRegenerate}
              className="p-1 rounded-full hover:bg-gray-200 focus:outline-none transition-colors"
              aria-label="Regenerate"
              tabIndex={0}
              title="Regenerate"
            >
              <Icon.RotateCcw className="h-4 w-4 text-gray-700" />
            </button>
          )}
          {onEdit && (
            <button
              onClick={onEdit}
              className="p-1 rounded-full hover:bg-gray-200 focus:outline-none transition-colors"
              aria-label="Regenerate"
              tabIndex={0}
              title="Regenerate"
            >
              <Icon.Edit3 className="h-4 w-4 text-gray-700" />
            </button>
          )}
          {onOk && (
            <button
              onClick={onOk}
              className="p-1 rounded-full hover:bg-gray-200 focus:outline-none transition-colors"
              aria-label="Regenerate"
              tabIndex={0}
              title="Regenerate"
            >
              <Icon.Save className="h-4 w-4 text-gray-700" />
            </button>
          )}
          {limitHeight !== undefined && (
            <KButton
              onClick={() => {
                setLimitHeight((prev) => {
                  return !prev;
                });
              }}
              className="p-1 rounded-full hover:bg-gray-200 focus:outline-none transition-colors"
              aria-label="show-full"
              tabIndex={0}
            >
              <Icon.Ellipsis className="h-4 w-4 text-gray-700" />
            </KButton>
          )}
          <button
            onClick={handleCopy}
            className="p-1 rounded-full hover:bg-gray-200 focus:outline-none transition-colors"
            aria-label="Copy"
            tabIndex={0}
            title="Copy"
          >
            <Icon.Copy className="h-4 w-4 text-gray-700" />
          </button>
        </div>
      </div>
    </div>
  );
};

export default RecordFrame;
