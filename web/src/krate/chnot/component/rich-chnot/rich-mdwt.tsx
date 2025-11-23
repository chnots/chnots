import { useCallback, useEffect, useRef, useState } from "react";
import useResizeObserver from "@react-hook/resize-observer";

import { chnotMetaList } from "../../service";
import MdwtChnot from "./mdwt";
import RichChnot, { type PostSaveArg } from "./rich-chnot";

import { useIsMobile } from "@/hooks/use-mobile";

import { cachedChnotMapByOtid } from "@/krate/chnot/store";
import { useKSpaceStore } from "@/krate/kspace/store";
import { arraysAreEqual } from "@/lib/col-util";
import type { TID } from "@/lib/id_util";
import { cn } from "@/lib/utils";
import useDebugChanged from "@/hooks/use-debug-changed";

const parseChnotsFromContent = (content: string): TID[] => {
  const regex = /\[\[([0-9]{16}?)\]\]/g;
  const matches: TID[] = [];

  // biome-ignore lint/suspicious/noImplicitAnyLet: old fashion
  let match;
  // biome-ignore lint/suspicious/noAssignInExpressions: old fashion
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
}: {
  onPostSave: (arg: PostSaveArg) => void;
  otid: TID;
  readonly?: boolean;
  content?: string;
  onChanged: (content: string) => void;
}) => {
  const { currentKSpace } = useKSpaceStore((e) => {
    return {
      currentKSpace: e.currentKSpace,
    };
  });

  const [chnots, setChnots] = useState<TID[]>([]);
  const isMobile = useIsMobile();

  const [_minHeight, setMinHeight] = useState<number | undefined>(200);
  const bodyRef = useRef<HTMLDivElement>(null);
  useResizeObserver<HTMLDivElement>(bodyRef, (entry) => {
    setMinHeight(entry.contentRect.height);
  });

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

  // biome-ignore lint/correctness/useExhaustiveDependencies: fixed
  useEffect(() => {
    updateChnots(initialContent);
  }, []);

  const handleContentChange = useCallback((content: string) => {
    updateChnots(content);
    onChanged(content);
  }, []);

  return (
    <div
      className={cn(
        "w-full p-1 m-1 h-full",
        isMobile || chnots.length === 0
          ? "flex flex-col divide-y max-w-4xl border"
          : "grid grid-cols-2 divide-x",
      )}
    >
      <MdwtChnot
        otid={otid}
        readonly={readonly}
        onPostSave={onPostSave}
        content={initialContent}
        onContentChange={handleContentChange}
        fullscreen={false}
        onSetFullscreen={() => {}}
      />
      {chnots.length > 0 && (
        <div className="space-y-2 rounded-none" ref={bodyRef}>
          {[...new Set(chnots)].map((otid) => (
            <RichChnot kspace={currentKSpace} otid={otid} key={otid} />
          ))}
        </div>
      )}
    </div>
  );
};

export default RichMdwt;
