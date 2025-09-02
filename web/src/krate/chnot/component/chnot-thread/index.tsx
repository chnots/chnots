import React, {
  RefObject,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import { ChnotKind, ChnotMeta, ChnotThreadMeta } from "../../po";
import { genTID, TID } from "@/lib/id_util";
import { ChnotOverwriteMetaReqData } from "../../dto";
import {
  chnotThreadMeta,
  chnotOverwriteMetas,
  chnotThreadOverwriteMeta,
} from "../../service";
import Chrome, { PostSaveArg } from "./chnot/chrome";
import LoadingPage from "@/common/pages/loading-page";
import { useChnotStore } from "../../store";
import { SaveState } from "@/common/types";
import { useShallow } from "zustand/react/shallow";
import { ar } from "date-fns/locale";
import { ChnotMetaKind } from "../vo";

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
  cachedThreadMetaRef: cachedThreadOtidRef,
  globalBar,
}: {
  threadMeta?: ChnotThreadMeta;
  cachedThreadMetaRef: RefObject<ChnotThreadMeta | null>;
  globalBar: React.ReactNode;
}) => {
  const cachedChnotDataMapRef = useRef<Map<TID, ChnotMetaKind>>(new Map());
  const savedChnotMetaMapRef = useRef<Map<TID, ChnotMeta>>(new Map());
  const [chnotOrders, setChnotOrders] = useState<TID[]>([]);

  const { overwriteChnotCache } = useChnotStore(
    useShallow((store) => {
      return {
        overwriteChnotCache: store.overwriteChnotCache,
      };
    }),
  );

  useEffect(() => {
    if (threadMeta) {
      chnotThreadMeta(threadMeta.otid).then((rsp) => {
        rsp.chnot_meta_sorted.forEach((meta) => {
          cachedChnotDataMapRef.current.set(meta.otid, meta);
        });
        savedChnotMetaMapRef.current = rsp.chnot_meta_sorted.reduce(function (
          map: Map<TID, ChnotMeta>,
          obj: ChnotMeta,
        ) {
          map.set(obj.otid, obj);
          return map;
        }, new Map());
        setChnotOrders([...rsp.chnot_meta_sorted.map((e) => e.otid), genTID()]);
      });
      cachedThreadOtidRef.current = threadMeta;
    } else {
      setChnotOrders([genTID()]);
    }
  }, []);

  if (
    chnotOrders.length === 0 ||
    cachedChnotDataMapRef.current.has(chnotOrders[chnotOrders.length - 1])
  ) {
    setChnotOrders((prev) => {
      return [...prev, genTID()];
    });
  }

  /**
   * We can only use this when the initial chnot OTIDs are present, since the `chnotOrders` is not empty.
   *
   * 1. try to save thread meta: check if threadMeta is undefined and otid is not cached(new chnotThread).
   * 2. check if chnot is persisted, so we can try to persist the chnot meta.
   * 3. check if persisted chnot is not same as the current listitem.
   */
  const saveMetas = useCallback(
    async (arg: PostSaveArg) => {
      if (!cachedThreadOtidRef.current) {
        const threadOtid = genTID();

        const rsp = await chnotThreadOverwriteMeta({
          meta_otid: threadOtid,
        });

        cachedThreadOtidRef.current = rsp.meta;

        overwriteChnotCache({
          meta: rsp.meta,
        });
      }
      if (arg.data) {
        cachedChnotDataMapRef.current.set(arg.data.otid, arg.data);
      }

      const metas: ChnotOverwriteMetaReqData[] = chnotOrders
        .map((otid, index) => {
          const persistedChnot = cachedChnotDataMapRef.current.get(otid);

          if (persistedChnot) {
            const saved = savedChnotMetaMapRef.current.get(otid);
            if (
              saved?.kind === persistedChnot.kind &&
              saved?.kind_id === persistedChnot.kind_id &&
              saved?.korder === index
            ) {
              return null;
            }
            return {
              otid: otid,
              korder: index,
              kind: persistedChnot.kind,
              kind_id: persistedChnot.kind_id,
            };
          }
          return null;
        })
        .filter((e) => e !== null);

      console.log("overwrite metas");
      if (metas.length > 0) {
        const rsp = await chnotOverwriteMetas({
          thread_otid: cachedThreadOtidRef.current!.otid,
          metas: metas,
        });
        for (const meta of rsp.metas) {
          savedChnotMetaMapRef.current.set(meta.otid, meta);
        }

        if (
          "kind" in arg &&
          arg.kind === ChnotKind.MDWT &&
          chnotOrders.at(0) === arg.data?.otid
        ) {
          overwriteChnotCache({
            meta: cachedThreadOtidRef.current,
            head_content: arg.content,
            todo_event: undefined,
          });
        }
      }

      if (
        savedChnotMetaMapRef.current.has(chnotOrders[chnotOrders.length - 1])
      ) {
        setChnotOrders([...chnotOrders, genTID()]);
      }
    },
    [chnotOrders],
  );

  console.log("rerender");
  return (
    <div className="flex flex-col w-full items-center overflow-y-auto">
      {chnotOrders.length == 0 ? (
        <LoadingPage />
      ) : (
        <>
          <div>{globalBar}</div>
          <div className="flex flex-col space-y-4 p-4 border m-2 w-full max-w-4xl">
            {chnotOrders.map((otid, index) => {
              return (
                <Chrome
                  key={otid}
                  onMoveUp={() => {}}
                  onMoveDown={() => {}}
                  onDelete={() => {}}
                  isFirst={index === 0}
                  isLast={index === chnotOrders.length - 1}
                  otid={otid}
                  onPostSave={(arg: PostSaveArg) => {
                    if (arg.saveState === SaveState.Saved) {
                      saveMetas(arg);
                    }
                  }}
                  meta={cachedChnotDataMapRef.current.get(otid)}
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
