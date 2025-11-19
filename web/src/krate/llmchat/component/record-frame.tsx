import { useCallback, useRef, useState } from 'react';
import clsx from 'clsx';

import type React from 'react';
import Icon from '@/common/component/icon';
import { Button } from '@/common/component/ui/button';

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
      variant={'ghost'}
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
  logo,
  children,
  justifyEnd,
  buttons,
  viewMode,
}: {
  name?: string;
  timestamp?: string;
  limitHeight?: boolean;
  justifyEnd?: boolean;
  onCopy?: () => void;
  logo?: React.ReactElement;
  children: React.ReactNode;
  buttons?: React.ReactNode;
  viewMode: boolean;
}) => {
  const contentRef = useRef<string>('');
  const handleCopy = useCallback(() => {
    if (onCopy) {
      onCopy();
    } else {
      navigator.clipboard.writeText(contentRef.current);
    }
  }, []);

  const [limitHeight, setLimitHeight] = useState<boolean | undefined>(initLimitHeight);

  return (
    <div
      className={clsx(
        'flex md:flex-row md:space-y-0 md:space-x-4 mx-4',
        justifyEnd && 'justify-end',
      )}
    >
      {logo && <>{logo}</>}
      <div className={clsx('flex-col')}>
        <div className="text-gray-500 text-xs space-x-2">
          <span>{name}</span>
          <span>{timestamp ?? 'Now'}</span>
        </div>
        {limitHeight != undefined && limitHeight ? (
          <div className={'max-h-160 overflow-hidden'}>{children}</div>
        ) : (
          <>{children}</>
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
                <Icon.Ellipsis className="h-4 w-4 text-gray-700" />
              </RecordButton>
            )}
            <RecordButton
              onClick={(): void => {
                handleCopy();
              }}
              children={<Icon.Copy />}
            />
          </div>
        )}
      </div>
    </div>
  );
};

export default RecordFrame;
