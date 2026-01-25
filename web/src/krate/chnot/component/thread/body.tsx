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
import { Plus } from "lucide-react";
import React, { useCallback, useEffect, useRef, useState } from "react";
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

enum ChnotState {
  Initialized,
  Saved,
}

const SortableRichMdwt = ({
  otid,
  onPostSave,
  content,
  onChanged,
  onAdd,
  index,
}: {
  otid: TID;
  onPostSave: (arg: PostSaveArg) => void;
  content: string;
  onChanged: () => void;
  onAdd: (position: number) => void;
  index: number;
}) => {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: otid });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  return (
    <div ref={setNodeRef} style={style} className="w-full" {...attributes}>
      <div className="flex items-start space-x-1">
        <div className="flex flex-col items-center mt-2 space-y-1">
          <div
            {...listeners}
            className="p-1 cursor-grab active:cursor-grabbing hover:bg-gray-100 rounded transition-colors"
            title="Drag Handler"
          >
            <svg
              className="w-4 h-4 text-gray-400"
              fill="currentColor"
              viewBox="0 0 20 20"
              aria-hidden="true"
            >
              <path d="M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z"></path>
            </svg>
          </div>
        </div>
        <div className="flex-1">
          <RichMdwt
            otid={otid}
            onPostSave={onPostSave}
            content={content}
            onChanged={onChanged}
          />
        </div>
      </div>
    </div>
  );
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
  const savedChnotMetaRef = useRef<Map<TID, ChnotState>>(new Map());
  const savedChnotOrdersRef = useRef<TID[]>([]);
  const savedChnotThreadMetaRef = useRef<ChnotThreadMeta>(undefined);
  const [chnotOrders, setChnotOrders] = useState<TID[]>([]);
  const [mdwtMap, setMdwtMap] = useState<Record<string, MdwtRecord>>({});
  const [loading, setLoading] = useState<boolean>(true);

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    }),
  );

  const handleDragEnd = useCallback((event: DragEndEvent) => {
    const { active, over } = event;

    if (active.id !== over?.id) {
      setChnotOrders((items) => {
        const oldIndex = items.indexOf(active.id as TID);
        const newIndex = items.indexOf(over?.id as TID);

        return arrayMove(items, oldIndex, newIndex);
      });
    }
  }, []);

  const handleAddBlock = useCallback((position: number) => {
    const newTid = genTID();
    setChnotOrders((prev) => {
      const newOrders = [...prev];
      newOrders.splice(position, 0, newTid);
      return newOrders;
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
            mdwt_otids: chnotOtids,
          });
          setChnotOrders(chnotOtids);
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
        const req: ChnotThreadMetaCommitReq = {
          meta_otid: threadMeta.otid,
          kspace: threadMeta.kspace,
        };
        const rsp = await chnotThreadMetaOverwrite(req);
        savedChnotThreadMetaRef.current = rsp.meta;
      }

      console.log("chnotMeta", savedChnotMetaRef, arg.otid);
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

      const toSaveChnotOrders = chnotOrders.filter(
        (e) => savedChnotMetaRef.current.get(e) === ChnotState.Saved,
      );
      if (!arraysAreEqual(toSaveChnotOrders, savedChnotOrdersRef.current)) {
        await chnotThreadOrderCommit({
          thread_otid: threadMeta.otid,
          orders: toSaveChnotOrders.map((e) => {
            return { otid: e };
          }),
        });
        savedChnotOrdersRef.current = toSaveChnotOrders;
      }
    },
    [chnotOrders, threadMeta],
  );

  return (
    <div className="flex flex-col w-full items-center overflow-y-auto">
      {loading ? (
        <LoadingPage />
      ) : (
        <DndContext
          sensors={sensors}
          collisionDetection={closestCenter}
          onDragEnd={handleDragEnd}
        >
          <div className="flex flex-col p-4 m-2 w-full max-w-4xl items-center">
            <SortableContext
              items={chnotOrders}
              strategy={verticalListSortingStrategy}
            >
              {chnotOrders.map((otid, index) => (
                <React.Fragment key={otid}>
                  <SortableRichMdwt
                    otid={otid}
                    onPostSave={(arg: PostSaveArg) => {
                      if (arg.saveState === SaveState.Saved) {
                        handlePostSaveOnChnot(arg);
                      }
                    }}
                    content={mdwtMap[otid]?.content ?? ""}
                    onChanged={(): void => {}}
                    onAdd={handleAddBlock}
                    index={index + 1}
                  />

                  <div className="flex items-center w-full">
                    <div className="flex justify-center w-8">
                      <button
                        type="button"
                        onClick={() => handleAddBlock(0)}
                        className="p-1 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded transition-colors"
                        title="添加"
                      >
                        <Plus className="w-4 h-4" />
                      </button>
                    </div>
                    <div className="flex-1 border-t border-gray-200 my-2"></div>
                  </div>
                </React.Fragment>
              ))}
            </SortableContext>
          </div>
        </DndContext>
      )}
    </div>
  );
};

export const ChnotThreadBodyMemo = React.memo(ChnotThreadBody);

export default ChnotThreadBody;
