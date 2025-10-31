import React, { useCallback, useEffect, useRef, useState } from "react";
import { ChnotThreadMeta } from "../../po";
import { genTID, TID } from "@/lib/id_util";
import { PostSaveArg } from "./chnot/rich-chnot";
import LoadingPage from "@/common/pages/loading-page";
import { useChnotStore } from "../../store";
import { SaveState } from "@/common/types";
import { useShallow } from "zustand/react/shallow";
import Chrome from "./chnot/chrome";
import {
  chnotThreadMetaFetch,
  chnotThreadMetaOverwrite,
  chnotThreadOrderCommit,
} from "../../service";
import { arraysAreEqual } from "@/lib/col-util";
import { mdwtRecordList } from "@/krate/mdwt/service";
import { MdwtRecord } from "@/krate/mdwt/po";

/**
 * This is the main component for the `Chnots` app.
 *
 * - Initial State
 *   - If the first `chnot` is `mdwt` type, just use it as the first `chnot`, which is called as `HEAD_CHNOT`.
 *   - If the first `chnot` is any other type(FOO), make the first chnot is `mdwt`, and create a new chnot, which type is FOO.
 *     - When we save the FOO chnot, initialize the HEAD_CHNOT and save it
 * - Maintain State
 *   - Just edit and save that chnot.
 */
const ChnotThread = ({
  threadMeta,
  globalBar,
}: {
  threadMeta: ChnotThreadMeta;
  globalBar: React.ReactNode;
}) => {
  console.log("render Thread: ", threadMeta?.otid);

  const { overwriteChnotCache } = useChnotStore((store) => {
    return {
      overwriteChnotCache: store.overwriteChnotCache,
    };
  });

  const savedChnotOrdersRef = useRef<TID[]>([]);
  const savedChnotThreadMetaRef = useRef<ChnotThreadMeta>(undefined);
  const [chnotOrders, setChnotOrders] = useState<TID[]>([]);
  const [mdwtMap, setMdwtMap] = useState<Record<string, MdwtRecord>>({});
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    (async () => {
      try {
        const rsp = await chnotThreadMetaFetch({
          thread_otid: threadMeta.otid,
        });

        if (rsp.thread_meta) {
          savedChnotThreadMetaRef.current = rsp.thread_meta;
        }

        if (rsp.chnot_meta_sorted.length == 0) {
          setChnotOrders([genTID()]);
        } else {
          const chnotOtids = rsp.chnot_meta_sorted.map((cm) => cm.otid);

          savedChnotOrdersRef.current = chnotOtids;

          const mdwtMap = await mdwtRecordList({
            mdwt_otids: chnotOtids,
          });
          setChnotOrders([...chnotOtids, genTID()]);
          setMdwtMap(mdwtMap.mdwt_map);
        }
      } finally {
        setLoading(false);
      }
    })();
  }, [threadMeta]);

  const handlePostSaveOnChnot = useCallback(
    async (arg: PostSaveArg) => {
      if (!savedChnotThreadMetaRef.current) {
        chnotThreadMetaOverwrite({
          meta_otid: threadMeta.otid,
          kspace: threadMeta.kspace,
        });
      }

      if (!arraysAreEqual(chnotOrders, savedChnotOrdersRef.current)) {
        chnotThreadOrderCommit({
          thread_otid: threadMeta.otid,
          orders: chnotOrders.map((e) => {
            return { otid: e };
          }),
        }).then((rsp) => {
          savedChnotOrdersRef.current = chnotOrders;
          setChnotOrders((prev) => {
            return [...prev, genTID()];
          });
        });
      }
    },
    [chnotOrders, threadMeta],
  );

  return (
    <div className="flex flex-col w-full items-center overflow-y-auto">
      {loading ? (
        <LoadingPage />
      ) : (
        <>
          <div>{globalBar}</div>
          <div className="flex flex-col space-y-1 p-4 m-2 w-full max-w-4xl items-center">
            {chnotOrders.map((otid, index) => {
              return (
                <Chrome
                  key={otid}
                  otid={otid}
                  onPostSave={(arg: PostSaveArg) => {
                    if (arg.saveState === SaveState.Saved) {
                      handlePostSaveOnChnot(arg);
                    }
                  }}
                  content={mdwtMap[otid]?.content ?? undefined}
                />
              );
            })}
          </div>
        </>
      )}
    </div>
  );
};

export default ChnotThread;
