import {
  closestCenter,
  DndContext,
  type DragEndEvent,
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
import LoadingPage from "@/common/pages/loading-page";
import { SaveState } from "@/common/types";
import type { MdwtRecord } from "@/krate/mdwt/po";
import { mdwtRecordList } from "@/krate/mdwt/service";
import { arraysAreEqual } from "@/lib/col-util";
import { genTID, type TID } from "@/lib/id_util";
import type { ChnotThreadMetaCommitReq } from "../../dto";
import { ChnotKind, type ChnotThreadMeta } from "../../po";
import {
  chnotMetaCommit,
  chnotThreadMetaFetch,
  chnotThreadMetaOverwrite,
  chnotThreadOrderCommit,
} from "../../service";
import type { PostSaveArg } from "../rich-chnot/rich-chnot";
import RichMdwt from "../rich-chnot/rich-mdwt";
import MdwtChnotSelector from "../rich-chnot/mdwt-chnot-selector";
import MdwtChnot from "../rich-chnot/mdwt";

enum ChnotState {
  Initialized,
  Saved,
}

const SortableRichMdwt = ({
  otid,
  onPostSave,
  content,
}: {
  otid: TID;
  onPostSave: (arg: PostSaveArg) => void;
  content: string;
}) => {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: otid });
  const [level, setLevel] = useState<number>(0);

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  const handlePostSave = useCallback((arg: PostSaveArg) => {
    if (arg.title) {
      const matches = arg.title.match(/^#+/g);
      const newLevel = matches?.[0]?.length ?? 0;
      if (matches && level !== newLevel) {
        setLevel(newLevel > 6 ? 0 : newLevel);
      }
    }

    onPostSave(arg);
  }, []);

  return (
    <div ref={setNodeRef} style={style} className="w-full" {...attributes}>
      <div className="flex items-start">
        <div className="flex items-center justify-center mt-1 w-8">
          <div
            {...listeners}
            className="p-1 cursor-grab active:cursor-grabbing hover:bg-gray-100 rounded transition-colors"
            title="Drag Handler"
          >
            {level === 1 ? (
              <Icon.Heading1 className="w-4 h-4" />
            ) : level === 2 ? (
              <Icon.Heading2 className="w-4 h-4" />
            ) : level === 3 ? (
              <Icon.Heading3 className="w-4 h-4" />
            ) : level === 4 ? (
              <Icon.Heading4 className="w-4 h-4" />
            ) : level === 5 ? (
              <Icon.Heading5 className="w-4 h-4" />
            ) : level === 6 ? (
              <Icon.Heading6 className="w-4 h-4" />
            ) : (
              <Icon.Text className="w-4 h-4" />
            )}
          </div>
        </div>
        <div className="flex-1">
          <RichMdwt otid={otid} onPostSave={handlePostSave} content={content} />
        </div>
      </div>
    </div>
  );
};

const SortableRichMdwtMemo = memo(SortableRichMdwt);

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
  const savedChnotMetaRef = useRef<Map<TID, ChnotState>>(new Map());
  const savedChnotOrdersRef = useRef<TID[]>([]);
  const savedChnotThreadMetaRef = useRef<ChnotThreadMeta>(undefined);
  const [chnotOrders, setChnotOrders] = useState<(TID | string)[]>([]);
  const [mdwtMap, setMdwtMap] = useState<Record<string, MdwtRecord>>({});
  const [loading, setLoading] = useState<boolean>(true);

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    }),
  );

  useEffect(() => {
    (async () => {
      const toSaveChnotOrders = chnotOrders
        .filter((e) => typeof e === "number")
        .filter((e) => savedChnotMetaRef.current.get(e) === ChnotState.Saved);
      if (!arraysAreEqual(toSaveChnotOrders, savedChnotOrdersRef.current)) {
        await chnotThreadOrderCommit({
          thread_otid: threadMeta.otid,
          orders: toSaveChnotOrders.map((e) => {
            return { otid: e };
          }),
        });
        savedChnotOrdersRef.current = toSaveChnotOrders;
      }
    })();
  }, [threadMeta, chnotOrders]);

  const handleDragEnd = useCallback((event: DragEndEvent) => {
    const { active, over } = event;

    if (active.id !== over?.id) {
      setChnotOrders((items) => {
        const oldIndex = items.indexOf(active.id as TID);
        const newIndex = items.indexOf(over?.id as TID);

        const newOrders = arrayMove(items, oldIndex, newIndex);
        return newOrders;
      });
    }
  }, []);

  const handleAddBlock = useCallback((position: number, find: boolean) => {
    setChnotOrders((prev) => {
      const newOrders = [...prev];
      newOrders.splice(position, 0, find ? "find-" + genTID() : genTID());
      return newOrders;
    });
  }, []);

  const handleRemoveBlock = useCallback((otid: TID | string) => {
    setChnotOrders((prev) => {
      const removed = prev.filter((e) => e !== otid);
      if (removed.length === 0) {
        return [genTID()];
      } else {
        return removed;
      }
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

        if (rsp.chnot_meta_sorted.length === 0) {
          setChnotOrders([genTID()]);
        } else {
          const chnotOtids = rsp.chnot_meta_sorted.map((cm) => cm.otid);

          savedChnotOrdersRef.current = chnotOtids;
          savedChnotMetaRef.current = new Map(
            chnotOtids.map((obj) => [obj, ChnotState.Saved]),
          );

          const mdwtMap = await mdwtRecordList({
            mdwt_otids: [...chnotOtids, threadMeta.otid],
          });
          setChnotOrders(chnotOtids);
          setMdwtMap(mdwtMap.mdwt_map);
        }
      } finally {
        setLoading(false);
      }
    })();
  }, [threadMeta]);

  const handleSearchAdd = useCallback(
    async (otidMap: Map<string, TID>) => {
      const mdwtMap = await mdwtRecordList({
        mdwt_otids: [...otidMap.values()],
      });
      if (Object.keys(mdwtMap.mdwt_map).length > 0) {
        setMdwtMap((prev) => {
          return { ...prev, ...mdwtMap.mdwt_map };
        });
        const savedMdwts = savedChnotMetaRef.current;
        for (const k in mdwtMap.mdwt_map) {
          savedMdwts.set(Number(k), ChnotState.Saved);
        }
        setChnotOrders((prev) => {
          return prev.map((e) => {
            if (typeof e === "string") {
              const otid = otidMap.get(e);
              if (otid) {
                return otid;
              } else {
                return e;
              }
            } else {
              return e;
            }
          });
        });
      }
    },
    [setMdwtMap],
  );

  const handlePostSaveOnChnot = useCallback(
    async (arg: PostSaveArg) => {
      if (arg.saveState !== SaveState.Saved) {
        return;
      }
      if (!savedChnotThreadMetaRef.current) {
        const req: ChnotThreadMetaCommitReq = {
          meta_otid: threadMeta.otid,
          kspace: threadMeta.kspace,
        };
        const rsp = await chnotThreadMetaOverwrite(req);
        savedChnotThreadMetaRef.current = rsp.meta;
      }

      if (savedChnotMetaRef.current.get(arg.otid) !== ChnotState.Saved) {
        await chnotMetaCommit({
          metas: [
            {
              otid: arg.otid,
              kind: ChnotKind.MDWT,
              kspace: threadMeta.kspace,
            },
          ],
        });
        savedChnotMetaRef.current.set(arg.otid, ChnotState.Saved);
      }
    },
    [chnotOrders, threadMeta],
  );

  return (
    <div className="flex flex-col w-full items-center overflow-y-auto">
      {loading ? (
        <LoadingPage />
      ) : (
        <div className="flex flex-col p-4 m-2 w-full items-center max-w-4xl">
          <div className="flex w-full">
            <div className="p-2">
              <Icon.Heading className="w-6 h-6" />
            </div>
            <MdwtChnot
              otid={threadMeta.otid}
              fullscreen={false}
              onPostSave={() => {}}
              content={
                mdwtMap[threadMeta.otid]?.content ?? `# Thread -- ${genTID()}`
              }
            />
          </div>
          <DndContext
            sensors={sensors}
            collisionDetection={closestCenter}
            onDragEnd={handleDragEnd}
          >
            <div className="flex flex-col w-full items-center">
              <SortableContext
                items={chnotOrders}
                strategy={verticalListSortingStrategy}
              >
                {chnotOrders.map((otid, index) => {
                  return typeof otid === "number" ? (
                    <React.Fragment key={otid}>
                      <div className="flex items-center w-full">
                        <div className="flex-1 border-t border-gray-200 my-2"></div>
                        <button
                          type="button"
                          onClick={() => handleRemoveBlock(otid)}
                          className="p-1 text-gray-400 hover:text-red-600 hover:bg-gray-100 rounded transition-colors"
                          title="Add"
                        >
                          <Icon.Trash2 className="w-4 h-4" />
                        </button>
                        <button
                          type="button"
                          onClick={() => handleAddBlock(index, false)}
                          className="p-1 text-gray-400 hover:text-green-600 hover:bg-gray-100 rounded transition-colors"
                          title="Add"
                        >
                          <Icon.Plus className="w-4 h-4" />
                        </button>
                        <button
                          type="button"
                          onClick={() => handleAddBlock(index, true)}
                          className="p-1 text-gray-400 hover:text-blue-600 hover:bg-gray-100 rounded transition-colors"
                          title="Add"
                        >
                          <Icon.Search className="w-4 h-4" />
                        </button>
                      </div>
                      <SortableRichMdwtMemo
                        otid={otid}
                        onPostSave={handlePostSaveOnChnot}
                        content={mdwtMap[otid]?.content ?? ""}
                      />
                    </React.Fragment>
                  ) : (
                    <React.Fragment key={otid}>
                      <div className="flex items-center w-full">
                        <div className="flex-1 border-t border-gray-200 my-2"></div>
                        <button
                          type="button"
                          onClick={() => handleRemoveBlock(otid)}
                          className="p-1 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded transition-colors"
                          title="Add"
                        >
                          <Icon.SearchXIcon className="w-4 h-4" />
                        </button>
                      </div>
                      <MdwtChnotSelector
                        key={otid}
                        onSelect={(tOtid) =>
                          handleSearchAdd(new Map([[otid, tOtid]]))
                        }
                      />
                    </React.Fragment>
                  );
                })}
              </SortableContext>
            </div>
          </DndContext>
        </div>
      )}
    </div>
  );
};

export const ChnotThreadBodyMemo = React.memo(ChnotThreadBody);

export default ChnotThreadBody;
