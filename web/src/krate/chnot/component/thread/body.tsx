import {
  closestCenter,
  closestCorners,
  DndContext,
  type DragEndEvent,
  DragOverlay,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import {
  arrayMove,
  SortableContext,
  sortableKeyboardCoordinates,
  useSortable,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import React, { memo, useCallback, useEffect, useRef, useState } from "react";
import Icon from "@/common/component/icon";
import { SaveState } from "@/common/types";
import type { MdwtRecord } from "@/krate/mdwt/po";
import { mdwtRecordList } from "@/krate/mdwt/service";
import { arraysAreEqual } from "@/lib/col-util";
import { genTID, type TID } from "@/lib/id_util";
import type { ChnotThreadMetaCommitReq } from "../../dto";
import type { ChnotKind, ChnotThreadMeta } from "../../po";
import {
  chnotMetaCommit,
  chnotThreadMetaFetch,
  chnotThreadMetaOverwrite,
  chnotThreadOrderCommit,
} from "../../service";
import { useChnotThreadStore } from "../../store";
import { ChnotKindIcon } from "../kind-icon";
import MdwtChnot from "../rich-chnot/mdwt";
import MdwtChnotSelector from "../rich-chnot/mdwt-chnot-selector";
import RichChnot, { type PostSaveArg } from "../rich-chnot/rich-mdwt-side";
import ChnotThreadSegment from "./segment";
import SortableRichMdwtMemo from "./segment";

enum ChnotState {
  Initialized,
  Saved,
}
type ChnotOrder = {
  otid: TID | string;
  chnotKind?: ChnotKind;
};

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
  const savedChnotOtidRef = useRef<Map<TID, ChnotState>>(new Map());
  const savedChnotOrdersRef = useRef<TID[]>([]);
  const savedChnotThreadMetaRef = useRef<ChnotThreadMeta>(undefined);
  const [chnotOrders, setChnotOrders] = useState<ChnotOrder[]>([]);
  const [mdwtMap, setMdwtMap] = useState<Record<string, MdwtRecord>>({});
  const [loading, setLoading] = useState<boolean>(true);
  const [activeId, setActiveId] = useState<TID | string | null>(null);
  const [overId, setOverId] = useState<TID | string | null>(null);

  const { overwrite } = useChnotThreadStore((store) => {
    return {
      overwrite: store.overwrite,
    };
  });

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    }),
  );

  useEffect(() => {
    (async () => {
      console.log("chnot order changed: ", chnotOrders);
      const toSaveChnotOrderOtids = chnotOrders
        .map((e) => e.otid)
        .filter((e) => typeof e === "number")
        .filter((e) => savedChnotOtidRef.current.get(e) === ChnotState.Saved);
      if (!arraysAreEqual(toSaveChnotOrderOtids, savedChnotOrdersRef.current)) {
        console.log(
          "chnot order commit chnotOrders: ",
          chnotOrders,
          savedChnotOrdersRef,
        );
        await chnotThreadOrderCommit({
          thread_otid: threadMeta.otid,
          orders: toSaveChnotOrderOtids.map((e) => {
            return { otid: e };
          }),
        });
        savedChnotOrdersRef.current = toSaveChnotOrderOtids;
      }
    })();
  }, [threadMeta, chnotOrders]);

  const handleDragStart = useCallback((event: DragEndEvent) => {
    setActiveId(event.active.id as TID | string);
  }, []);

  const handleDragOver = useCallback((event: DragEndEvent) => {
    setOverId(event.over?.id as TID | string | null);
  }, []);

  const handleDragEnd = useCallback((event: DragEndEvent) => {
    const { active, over } = event;

    setActiveId(null);
    setOverId(null);

    if (active.id !== over?.id) {
      setChnotOrders((items) => {
        const oldIndex = items.findIndex((e) => e.otid === (active.id as TID));
        const newIndex = items.findIndex((e) => e.otid === (over?.id as TID));

        if (oldIndex !== undefined && newIndex !== undefined) {
          return arrayMove(items, oldIndex, newIndex);
        } else {
          return items;
        }
      });
    }
  }, []);

  const handleAddBlock = useCallback((position: number, find: boolean) => {
    setChnotOrders((prev) => {
      const newOrders = [...prev];
      newOrders.splice(position, 0, {
        otid: find ? "find-" + genTID() : genTID(),
      });
      return newOrders;
    });
  }, []);

  const handleRemoveBlock = useCallback((otid: TID | string) => {
    setChnotOrders((prev) => {
      const removed = prev.filter((e) => e.otid !== otid);
      return removed;
    });
  }, []);

  useEffect(() => {
    (async () => {
      try {
        const rsp = await chnotThreadMetaFetch({
          thread_otid: threadMeta.otid,
        });

        if (rsp.thread_meta) {
          savedChnotThreadMetaRef.current = rsp.thread_meta;
        }

        const chnotOtids = rsp.chnot_meta_sorted.map((cm) => {
          return {
            otid: cm.otid,
            chnotKind: cm.kind,
          };
        });

        savedChnotOrdersRef.current = chnotOtids.map((e) => e.otid);
        savedChnotOtidRef.current = new Map(
          chnotOtids.map((obj) => [obj.otid, ChnotState.Saved]),
        );

        const mdwtMap = await mdwtRecordList({
          mdwt_otids: [...savedChnotOrdersRef.current, threadMeta.otid],
        });
        setChnotOrders(chnotOtids);
        setMdwtMap(mdwtMap.mdwt_map);
      } finally {
        setLoading(false);
      }
    })();
  }, [threadMeta]);

  const handleSearchAdd = useCallback(
    async (old: string, otid: TID, kind: ChnotKind) => {
      const mdwtMap = await mdwtRecordList({
        mdwt_otids: [otid],
      });
      if (Object.keys(mdwtMap.mdwt_map).length > 0) {
        setMdwtMap((prev) => {
          return { ...prev, ...mdwtMap.mdwt_map };
        });
        const savedMdwts = savedChnotOtidRef.current;
        for (const k in mdwtMap.mdwt_map) {
          savedMdwts.set(Number(k), ChnotState.Saved);
        }
        setChnotOrders((prev) => {
          return prev.map((e) => {
            if (typeof e === "string" && old === e) {
              return { chnotKind: kind, otid: otid };
            } else {
              return e;
            }
          });
        });
      }
    },
    [setMdwtMap],
  );

  const handleSaveThreadMeta = useCallback(async () => {
    if (!savedChnotThreadMetaRef.current) {
      const req: ChnotThreadMetaCommitReq = {
        meta_otid: threadMeta.otid,
        kspace: threadMeta.kspace,
      };
      const rsp = await chnotThreadMetaOverwrite(req);
      savedChnotThreadMetaRef.current = rsp.meta;
    }
  }, [savedChnotThreadMetaRef]);

  const handlePostSaveOnChnot = useCallback(
    async (arg: PostSaveArg) => {
      if (arg.saveState !== SaveState.Saved) {
        return;
      }

      savedChnotOtidRef.current.set(arg.otid, ChnotState.Saved);

      // force update.
      setChnotOrders((prev) => [...prev]);
    },
    [chnotOrders, threadMeta],
  );

  const titleRef = useRef<string>(null);
  const handleTitleSave = useCallback(
    async (arg: PostSaveArg) => {
      const meta = threadMeta;
      await handleSaveThreadMeta();
      if (arg.title !== titleRef.current) {
        const title = arg.title ?? "";
        titleRef.current = title;

        overwrite({
          meta: meta,
          title: titleRef.current ?? undefined,
        });
      }
    },
    [threadMeta],
  );

  return (
    <div className="flex flex-col w-full items-center overflow-y-auto">
      {loading ? (
        <div />
      ) : (
        <div className="flex flex-col p-4 m-2 w-full items-center max-w-4xl">
          <div className="flex w-full">
            <div className="py-2 pr-2">
              <Icon.Heading className="w-6 h-6" />
            </div>
            <MdwtChnot
              otid={threadMeta.otid}
              fullscreen={false}
              onPostSave={handleTitleSave}
              content={
                mdwtMap[threadMeta.otid]?.content ?? `# Thread -- ${genTID()}`
              }
            />
          </div>
          <DndContext
            sensors={sensors}
            collisionDetection={closestCorners}
            onDragStart={handleDragStart}
            onDragEnd={handleDragEnd}
            onDragOver={handleDragOver}
          >
            <div className="flex flex-col w-full items-center">
              <SortableContext
                items={chnotOrders.map((e) => e.otid)}
                strategy={verticalListSortingStrategy}
              >
                {chnotOrders.map((order, index) => {
                  return typeof order.otid === "number" ? (
                    <React.Fragment key={order.otid}>
                      {activeId !== null && overId === order.otid && (
                        <div className="w-full h-0.5 bg-blue-500 my-1" />
                      )}
                      <SortableRichMdwtMemo
                        otid={order.otid}
                        index={index}
                        kind={order.chnotKind}
                        onPostSave={handlePostSaveOnChnot}
                        content={mdwtMap[order.otid]?.content ?? ""}
                        kspace={threadMeta.kspace}
                        handleAddBlock={handleAddBlock}
                        handleRemoveBlock={handleRemoveBlock}
                        isDragging={activeId === order.otid}
                      />
                    </React.Fragment>
                  ) : (
                    <React.Fragment key={order.otid}>
                      <div className="flex items-center w-full">
                        <div className="flex-1 border-t border-gray-200 my-2"></div>
                        <button
                          type="button"
                          onClick={() => handleRemoveBlock(order.otid)}
                          className="p-1 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded transition-colors"
                          title="Add"
                        >
                          <Icon.SearchXIcon className="w-4 h-4" />
                        </button>
                      </div>
                      <MdwtChnotSelector
                        key={order.otid}
                        onSelect={(tOtid, kind) =>
                          typeof order.otid === "string" &&
                          handleSearchAdd(order.otid, tOtid, kind)
                        }
                      />
                    </React.Fragment>
                  );
                })}
                <div className="flex text-gray-400 items-center w-full">
                  <div className="flex-1 border-t border-gray-200 my-2"></div>
                  <button
                    type="button"
                    onClick={() => handleAddBlock(chnotOrders.length + 1, true)}
                    className="p-1 hover:text-blue-600 hover:bg-gray-100 rounded transition-colors"
                    title="Add"
                  >
                    <Icon.ZoomIn className="w-4 h-4" />
                  </button>
                  <button
                    type="button"
                    onClick={() =>
                      handleAddBlock(chnotOrders.length + 1, false)
                    }
                    className="p-1 hover:text-green-600 hover:bg-gray-100 rounded transition-colors"
                    title="Add"
                  >
                    <Icon.Plus className="w-4 h-4" />
                  </button>
                </div>
              </SortableContext>
            </div>
            <DragOverlay>
              {activeId !== null ? (
                <div className="w-full h-0.5 bg-blue-500" />
              ) : null}
            </DragOverlay>
          </DndContext>
        </div>
      )}
    </div>
  );
};

export const ChnotThreadBodyMemo = React.memo(ChnotThreadBody);

export default ChnotThreadBody;
