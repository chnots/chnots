import React, { useCallback, useEffect, useRef, useState } from 'react';

import {
  chnotThreadMetaFetch,
  chnotThreadMetaOverwrite,
  chnotThreadOrderCommit,
} from '../../service';
import RichMdwt from '../rich-chnot/rich-mdwt';

import type { ChnotThreadMeta } from '../../po';
import type { PostSaveArg } from '../rich-chnot/rich-chnot';
import LoadingPage from '@/common/pages/loading-page';
import { SaveState } from '@/common/types';
import type { MdwtRecord } from '@/krate/mdwt/po';
import { mdwtRecordList } from '@/krate/mdwt/service';
import { arraysAreEqual } from '@/lib/col-util';
import { genTID, type TID } from '@/lib/id_util';

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
const ChnotThreadBody = ({ threadMeta }: { threadMeta: ChnotThreadMeta }) => {
  const savedChnotOrdersRef = useRef<TID[]>([]);
  const savedChnotThreadMetaRef = useRef<ChnotThreadMeta>(undefined);
  const initializedOtids = useRef<Set<TID>>(new Set());
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

        if (rsp.chnot_meta_sorted.length === 0) {
          setChnotOrders([genTID()]);
        } else {
          const chnotOtids = rsp.chnot_meta_sorted.map((cm) => cm.otid);
          initializedOtids.current.union(new Set(chnotOtids));

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
    async (_arg: PostSaveArg) => {
      if (!savedChnotThreadMetaRef.current) {
        chnotThreadMetaOverwrite({
          meta_otid: threadMeta.otid,
          kspace: threadMeta.kspace,
        });
      }

      if (!arraysAreEqual(chnotOrders, savedChnotOrdersRef.current)) {
        chnotThreadOrderCommit({
          thread_otid: threadMeta.otid,
          orders: chnotOrders
            .filter((otid) => initializedOtids.current.has(otid))
            .map((e) => {
              return { otid: e };
            }),
        }).then((_rsp) => {
          savedChnotOrdersRef.current = chnotOrders;
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
        <div className="flex flex-col space-y-1 p-4 m-2 w-full max-w-4xl items-center">
          {chnotOrders.map((otid, _index) => {
            return (
              <RichMdwt
                key={otid}
                otid={otid}
                onPostSave={(arg: PostSaveArg) => {
                  if (arg.saveState === SaveState.Saved) {
                    handlePostSaveOnChnot(arg);
                  }
                }}
                content={mdwtMap[otid]?.content ?? undefined}
                onChanged={(): void => {
                  const inited = initializedOtids.current;
                  /**
                   * if this otid is not added, we think maybe we should add a new chnot to the end of chnot-thread.
                   *
                   * try to add changed otid
                   */
                  if (!inited.has(otid)) {
                    inited.add(otid);

                    const lastOtid = chnotOrders.at(chnotOrders.length - 1);
                    if (lastOtid && inited.has(lastOtid)) {
                      setChnotOrders((prev) => {
                        return [...prev, genTID()];
                      });
                    }
                  }
                }}
                tryfetch={false}
              />
            );
          })}
        </div>
      )}
    </div>
  );
};

export const ChnotThreadBodyMemo = React.memo(ChnotThreadBody);

export default ChnotThreadBody;
