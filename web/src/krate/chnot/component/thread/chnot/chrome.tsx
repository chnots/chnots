import { TID } from "@/lib/id_util";
import RichChnot, { PostSaveArg } from "./rich-chnot";
import MdwtRecord from "./mdwt";
import { useEffect, useState } from "react";
import { arraysAreEqual } from "@/lib/col-util";
import { chnotMetaCommit, chnotMetaList } from "../../../service";
import { ChnotKind, ChnotMeta } from "../../../po";
import { SaveState } from "@/common/types";
import { useKSpaceStore } from "@/krate/kspace/store";
import Icon from "@/common/component/icon";

const RightSide = ({}: {}) => {};

const parseChnotsFromContent = (content: string): TID[] => {
  const regex = /\[\[([0-9]*?)\]\]/g;
  const matches: TID[] = [];
  let match;

  while ((match = regex.exec(content)) !== null) {
    matches.push(parseInt(match[1], 10));
  }

  return matches;
};

const Chrome = ({
  onPostSave,
  otid,
  readonly,
  content: initialContent,
}: {
  onPostSave: (arg: PostSaveArg) => void;
  otid: TID;
  readonly?: boolean;
  content?: string;
}) => {
  console.log("render Chrome: ", otid);
  const [chnots, setChnots] = useState<TID[]>([]);
  const [chnotMetas, setChnotMetas] = useState<ChnotMeta[]>();
  const { currentKSpace } = useKSpaceStore((e) => {
    {
      return {
        currentKSpace: e.currentKSpace,
      };
    }
  });
  useEffect(() => {
    if (initialContent) {
      setChnots(parseChnotsFromContent(initialContent));
    }
  }, []);
  useEffect(() => {
    (async () => {
      const chnotMetas = await chnotMetaList({ otids: chnots });
      setChnotMetas(chnotMetas.metas);
    })();
  }, [chnots]);

  return (
    <div className="grid grid-cols-2 border-b-1">
      <div className="h-full w-full">
        <MdwtRecord
          otid={otid}
          readonly={readonly}
          onPostSave={(arg: PostSaveArg) => {
            if (arg.saveState === SaveState.Saved) {
              chnotMetaCommit({
                metas: [
                  {
                    otid: otid,
                    kind: ChnotKind.MDWT,
                    kspace: currentKSpace,
                  },
                ],
              });
            }
            onPostSave(arg);
          }}
          content={initialContent}
          onContentChange={(content) => {
            const links = parseChnotsFromContent(content);
            if (!arraysAreEqual(links, chnots)) {
              setChnots(chnots);
            }
          }}
        />
      </div>
      {chnotMetas && (
        <div>
          {chnotMetas.map((cm) => (
            <div key={cm.otid}>
              <RichChnot
                meta={{
                  chnotOtid: cm.otid,
                  kind: cm.kind,
                }}
              />
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default Chrome;
