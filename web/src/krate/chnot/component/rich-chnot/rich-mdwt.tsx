import { useCallback, useEffect, useRef, useState } from 'react';
import useResizeObserver from '@react-hook/resize-observer';

import { chnotMetaList } from '../../service';
import MdwtChnot from './mdwt';
import RichChnot, { type PostSaveArg } from './rich-chnot';

import { useIsMobile } from '@/hooks/use-mobile';

import { cachedChnotMapByOtid } from '@/krate/chnot/store';
import { useKSpaceStore } from '@/krate/kspace/store';
import { arraysAreEqual } from '@/lib/col-util';
import type { TID } from '@/lib/id_util';
import { cn } from '@/lib/utils';

const parseChnotsFromContent = (content: string): TID[] => {
  const regex = /\[\[([0-9]{16}?)\]\]/g;
  const matches: TID[] = [];
  let match;

  while ((match = regex.exec(content)) !== null) {
    matches.push(parseInt(match[1], 10));
  }

  return matches;
};

const RichMdwt = ({
  onPostSave,
  otid,
  readonly,
  content: initialContent,
  onChanged,
  whfull,
  tryfetch,
}: {
  onPostSave: (arg: PostSaveArg) => void;
  otid: TID;
  readonly?: boolean;
  content?: string;
  onChanged: (content: string) => void;
  whfull?: string;
  tryfetch: boolean;
}) => {
  console.log('render Chrome: ', otid);

  const { currentKSpace } = useKSpaceStore((e) => {
    return {
      currentKSpace: e.currentKSpace,
    };
  });

  const [chnots, setChnots] = useState<TID[]>([]);
  const isMobile = useIsMobile();

  const [minHeight, setMinHeight] = useState<number | undefined>(200);
  const bodyRef = useRef<HTMLDivElement>(null);
  useResizeObserver<HTMLDivElement>(bodyRef, (entry) => {
    console.log('set minheight', entry.contentRect.height);
    setMinHeight(entry.contentRect.height);
  });
  console.log(minHeight);

  const updateChnots = useCallback(
    async (content?: string) => {
      const newChnots = content ? parseChnotsFromContent(content) : [];
      if (!arraysAreEqual(chnots, newChnots)) {
        const chnotMetas = await chnotMetaList({ otids: newChnots });
        if (chnotMetas.metas.length > 0) {
          chnotMetas.metas.forEach((cm) => {
            cachedChnotMapByOtid.set(cm.otid, cm);
          });
        }
        setChnots(newChnots);
      }
    },
    [chnots],
  );
  useEffect(() => {
    updateChnots(initialContent);
  }, [updateChnots]);

  console.log('rich chnots:', chnots);
  return (
    <div
      className={cn(
        'h-full w-full max-w-4xl border p-1 m-1 rounded',
        isMobile || chnots.length == 0 ? 'flex flex-col divide-y' : 'grid grid-cols-2 divide-x',
        whfull,
      )}
    >
      <MdwtChnot
        tryFetch={tryfetch}
        otid={otid}
        readonly={readonly}
        onPostSave={(arg: PostSaveArg) => {
          onPostSave(arg);
        }}
        content={initialContent}
        onContentChange={(content) => {
          updateChnots(content);
          onChanged(content);
        }}
        fullscreen={false}
        onSetFullscreen={() => {}}
      />
      {chnots.length > 0 && (
        <div className="space-y-2 rounded-none" ref={bodyRef}>
          {chnots.map((otid) => (
            <RichChnot kspace={currentKSpace} otid={otid} key={otid} />
          ))}
        </div>
      )}
    </div>
  );
};

export default RichMdwt;
