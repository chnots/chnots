import { TID } from "@/lib/id_util";
import RichChnot, { PostSaveArg } from "./rich-chnot";
import MdwtChnot from "./mdwt";
import { useCallback, useEffect, useState } from "react";
import { arraysAreEqual } from "@/lib/col-util";
import { chnotMetaCommit, chnotMetaList } from "../../service";
import { ChnotKind } from "../../po";
import { SaveState } from "@/common/types";
import { useKSpaceStore } from "@/krate/kspace/store";
import { cachedChnotMapByOtid } from "@/krate/chnot/store";
import { useIsMobile } from "@/hooks/use-mobile";
import clsx from "clsx";

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
  console.log("render Chrome: ", otid);

  const { currentKSpace } = useKSpaceStore((e) => {
    return {
      currentKSpace: e.currentKSpace,
    };
  });

  const [chnots, setChnots] = useState<TID[]>([]);
  const isMobile = useIsMobile();

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
  }, []);

  return (
    <div
      className={clsx(
        "w-full max-w-4xl border p-1 m-1 rounded",
        isMobile || chnots.length == 0 ? "" : "grid grid-cols-2",
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
        <div>
          {chnots.map((cm) => (
            <RichChnot kspace={currentKSpace} otid={cm} key={cm} />
          ))}
        </div>
      )}
    </div>
  );
};

export default RichMdwt;
