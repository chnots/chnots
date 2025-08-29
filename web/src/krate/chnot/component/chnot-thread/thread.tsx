import { useCallback, useEffect, useRef, useState } from "react";
import ChnotBlockChrome from "./chnot/chrome";
import { genTID, TID } from "@/lib/id_util";
import { chnotMeta, chnotOverwriteBlockMetas } from "../../service";
import { ChnotMetaKind } from "../vo";
import { ChnotMeta } from "../../po";
import { ChnotOverwriteMetaReqData } from "../../dto";

const ChnotBlocks = ({ threadOtid }: { threadOtid: TID }) => {
  const blockKinds = useRef<Map<TID, ChnotMetaKind>>(new Map());
  const [blockOrders, setBlockOrders] = useState<TID[]>([]);
  const savedOrders = useRef<ChnotMeta[]>([]);

  useEffect(() => {
    chnotMeta(threadOtid).then((rsp) => {
      rsp.chnot_meta_sorted.forEach((meta) => {
        blockKinds.current.set(meta.otid, meta);
      });
      setBlockOrders([...rsp.chnot_meta_sorted.map((e) => e.otid), genTID()]);
    });
  }, []);

  if (
    blockOrders.length === 0 ||
    blockKinds.current.has(blockOrders[blockOrders.length - 1])
  ) {
    setBlockOrders((prev) => {
      return [...prev, genTID()];
    });
  }

  const saveMetas = useCallback(() => {
    const savedOrdersCurrent = savedOrders.current;
    const toSave: ChnotOverwriteMetaReqData[] = blockOrders
      .map((otid, index) => {
        const saved = savedOrdersCurrent.at(index);
        const create = blockKinds.current.get(otid);
        if (
          saved?.kind === create?.kind &&
          saved?.kind_id === create?.kind_id &&
          saved?.korder === index
        ) {
          return null;
        }
        if (create) {
          return {
            otid: otid,
            korder: index,
            kind: create.kind,
            kind_id: create.kind_id,
            tid: genTID(),
          };
        }
        return null;
      })
      .filter((e) => e !== null);

    chnotOverwriteBlockMetas({
      thread_otid: threadOtid,
      metas: toSave,
    });
    if (blockKinds.current.has(blockOrders[blockOrders.length - 1])) {
      setBlockOrders((prev) => {
        return [...prev, genTID()];
      });
    }
  }, [blockOrders]);

  return (
    <div className="flex flex-col space-y-4 p-4 border m-2 w-full max-w-4xl">
      {blockOrders.map((otid, index) => {
        return (
          <ChnotBlockChrome
            key={otid}
            onMoveUp={() => {}}
            onMoveDown={() => {}}
            onDelete={() => {}}
            isFirst={index === 0}
            isLast={index === blockOrders.length - 1}
            blockKindsRef={blockKinds}
            otid={otid}
            onSaved={function (): void {
              saveMetas();
            }}
          />
        );
      })}
    </div>
  );
};

export default ChnotBlocks;
