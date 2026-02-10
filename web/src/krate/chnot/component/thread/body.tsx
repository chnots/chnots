import {
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
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import React, { useCallback, useEffect, useRef, useState } from "react";
import Icon from "@/common/component/icon";
import { SaveState } from "@/common/types";
import type { MdwtRecord } from "@/krate/mdwt/po";
import { mdwtRecordList } from "@/krate/mdwt/service";
import { arraysAreEqual } from "@/lib/col-util";
import { genTID, type TID } from "@/lib/id_util";
import type { ChnotThreadMetaCommitReq } from "../../dto";
import type { ChnotKind, ChnotThreadMeta } from "../../po";
import {
  chnotThreadMetaFetch,
  chnotThreadMetaOverwrite,
  chnotThreadOrderCommit,
} from "../../service";
import { useChnotThreadStore } from "../../store";
import MdwtChnot from "../rich-chnot/mdwt";
import MdwtChnotSelector from "../rich-chnot/mdwt-chnot-selector";
import type { PostSaveArg } from "../rich-chnot/rich-mdwt-side";
import SortableRichMdwtMemo from "./segment";

enum OrderType {
  Manual,
  Search,
}

type SavedChnotOrder = {
  type: OrderType.Manual;
  otid: TID;
  chnotKind: ChnotKind;
  closed: boolean;
};

type ManualChnotOrder = {
  type: OrderType.Manual;
  otid: TID;
  chnotKind?: ChnotKind;
  closed: boolean;
  saved: boolean;
};

type ChnotOrder = ManualChnotOrder | { type: OrderType.Search; otid: string };

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
  const savedChnotOrdersRef = useRef<ManualChnotOrder[]>([]);
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
      const toSaveChnotOrderOtids: ManualChnotOrder[] = chnotOrders
        .filter((e) => e.type === OrderType.Manual)
        .filter((e) => e.saved);
      if (
        !arraysAreEqual(
          toSaveChnotOrderOtids,
          savedChnotOrdersRef.current,
          (v1, v2) => {
            return (
              v1.otid === v2.otid &&
              v1.chnotKind === v2.chnotKind &&
              v1.closed === v2.closed
            );
          },
        )
      ) {
        await handleSaveThreadMeta();
        await chnotThreadOrderCommit({
          thread_otid: threadMeta.otid,
          orders: toSaveChnotOrderOtids.map((e) => {
            return { otid: e.otid, closed: e.closed };
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
      newOrders.splice(
        position,
        0,
        find
          ? {
              otid: "find-" + genTID(),
              type: OrderType.Search,
            }
          : {
              otid: genTID(),
              type: OrderType.Manual,
              closed: false,
              saved: false,
            },
      );
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

        const chnotOtids: SavedChnotOrder[] = rsp.chnot_meta_sorted.map(
          (cm) => {
            return {
              otid: cm.meta.otid,
              chnotKind: cm.meta.kind,
              closed: cm.closed,
              type: OrderType.Manual,
            };
          },
        );

        savedChnotOrdersRef.current = chnotOtids.map((e) => ({
          ...e,
          saved: true,
        }));

        const mdwtMap = await mdwtRecordList({
          mdwt_otids: [
            ...savedChnotOrdersRef.current.map((e) => e.otid),
            threadMeta.otid,
          ],
        });
        if (chnotOtids.length > 0) {
          setChnotOrders(savedChnotOrdersRef.current);
          setMdwtMap(mdwtMap.mdwt_map);
        } else {
          setChnotOrders([
            {
              otid: genTID(),
              type: OrderType.Manual,
              closed: false,
              saved: false,
            },
          ]);
        }
      } finally {
        setLoading(false);
      }
    })();
  }, [threadMeta]);

  const handleSearchAdd = useCallback(
    async (old: string, otid: TID, kind: ChnotKind) => {
      console.log("handle search add", old, otid);
      const mdwtMap = await mdwtRecordList({
        mdwt_otids: [otid],
      });
      setMdwtMap((prev) => {
        return { ...prev, ...mdwtMap.mdwt_map };
      });
      setChnotOrders((prev) => {
        return prev.map((e) => {
          if (e.type === OrderType.Search && old === e.otid) {
            return {
              chnotKind: kind,
              otid: otid,
              closed: false,
              type: OrderType.Manual,
              saved: true,
            };
          } else {
            return e;
          }
        });
      });
    },
    [],
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
  }, []);

  const handlePostSaveOnChnot = useCallback(
    async (arg: PostSaveArg) => {
      if (arg.saveState !== SaveState.Saved) {
        return;
      }
      // force update.
      setChnotOrders((prev) =>
        prev.map((e) => {
          if (e.type === OrderType.Manual && e.otid === arg.otid) {
            return { ...e, saved: true };
          } else {
            return e;
          }
        }),
      );
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

  const handleToggleClosed = useCallback(
    async (index: number, closed: boolean) => {
      setChnotOrders((prev) =>
        prev.map((co, id) => {
          if (index === id) {
            return { ...co, closed };
          } else {
            return co;
          }
        }),
      );
    },
    [],
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
              content={mdwtMap[threadMeta.otid]?.content ?? ""}
              placeholder="Thread Title"
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
                  return order.type === OrderType.Manual ? (
                    <React.Fragment key={order.otid}>
                      {activeId !== null && overId === order.otid && (
                        <div className="w-full h-0.5 bg-blue-500 my-1" />
                      )}
                      <SortableRichMdwtMemo
                        otid={order.otid}
                        index={index}
                        closed={order.closed}
                        kind={order.chnotKind}
                        onPostSave={handlePostSaveOnChnot}
                        content={mdwtMap[order.otid]?.content ?? ""}
                        kspace={threadMeta.kspace}
                        onAddBlock={handleAddBlock}
                        onRemoveBlock={handleRemoveBlock}
                        isDragging={activeId === order.otid}
                        onToggleClosed={handleToggleClosed}
                        saveState={
                          order.saved ? SaveState.Saved : SaveState.Initial
                        }
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
                          title="Stop Find"
                        >
                          <Icon.LucideUnlink className="w-4 h-4" />
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
                    <Icon.LinkIcon className="w-4 h-4" />
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
