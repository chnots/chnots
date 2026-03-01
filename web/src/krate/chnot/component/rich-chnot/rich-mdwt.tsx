import { useCallback, useRef } from "react";
import { useIsMobile } from "@/hooks/use-mobile";
import { useStateWithRef } from "@/hooks/use-state-ref";
import { cachedChnotMapByOtid } from "@/krate/chnot/store";
import { useKSpaceStore } from "@/krate/kspace/store";
import { arraysAreEqual } from "@/lib/col-util";
import type { TID } from "@/lib/id_util";
import { cn } from "@/lib/utils";
import { chnotMetaList } from "../../service";
import MdwtChnot from "./mdwt";
import RichChnot, { type PostSaveArg } from "./rich-mdwt-side";

const parseChnotsFromContent = (content: string): TID[] => {
  const regex = /\[\[([0-9]{16})\]\]/g;
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
}: {
  onPostSave: (arg: PostSaveArg) => Promise<void>;
  otid: TID;
  readonly?: boolean;
  content?: string;
}) => {
  const { currentKSpace } = useKSpaceStore((e) => {
    return {
      currentKSpace: e.currentKSpace,
    };
  });

  console.log("content", initialContent);
  const [chnots, setChnots, chnotsRef] = useStateWithRef<TID[]>(
    initialContent ? parseChnotsFromContent(initialContent) : [],
  );
  const isMobile = useIsMobile();
  const bodyRef = useRef<HTMLDivElement>(null);

  const handleContentChange = useCallback(async (content?: string) => {
    const newChnots = content ? parseChnotsFromContent(content) : [];
    if (!arraysAreEqual(chnotsRef.current, newChnots)) {
      const chnotMetas = await chnotMetaList({ otids: newChnots });
      if (chnotMetas.metas.length > 0) {
        chnotMetas.metas.forEach((cm) => {
          cachedChnotMapByOtid.set(cm.otid, cm);
        });
      }

      setChnots(newChnots);
    }
  }, []);

  return (
    <div
      className={cn(
        "w-full p-1 h-full",
        isMobile || chnots.length === 0
          ? "flex flex-col divide-y max-w-4xl"
          : "grid grid-cols-2 divide-x",
      )}
    >
      <MdwtChnot
        otid={otid}
        readonly={readonly}
        onPostSave={onPostSave}
        content={initialContent}
        fullscreen={false}
        onSetFullscreen={() => {}}
        onContentChange={handleContentChange}
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
