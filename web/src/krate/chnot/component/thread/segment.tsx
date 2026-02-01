import { mdwtCommit } from "@/krate/mdwt/service";
import ExcalidrawChnot from "../rich-chnot/excalidraw";
import KFileChnot from "../rich-chnot/kfile";
import LLMChatChnot from "../rich-chnot/llmchat";
import RichMdwt from "../rich-chnot/rich-mdwt";
import TableChnot from "../rich-chnot/table";
import MindMapChnot from "../rich-chnot/mindmap";

import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import React, { memo, useCallback, useMemo, useRef, useState } from "react";
import Icon from "@/common/component/icon";
import { SaveState } from "@/common/types";
import { genTID, type TID } from "@/lib/id_util";
import { ChnotKind } from "../../po";
import { chnotMetaCommit } from "../../service";
import type { PostSaveArg } from "../rich-chnot/rich-mdwt-side";
import { ChnotKindIcon } from "../kind-icon";

const SortableRichMdwt = ({
  otid,
  index,
  onPostSave,
  handleAddBlock,
  handleRemoveBlock,
  content,
  kspace,
  kind: initialKind,
}: {
  otid: TID;
  index: number;
  onPostSave: (arg: PostSaveArg) => void;
  content: string;
  kspace: string;
  kind?: ChnotKind;
  handleAddBlock: (position: number, find: boolean) => void;
  handleRemoveBlock: (otid: string | TID) => void;
}) => {
  const saveStateRef = useRef<SaveState>(SaveState.Initial);
  const titleRef = useRef<string | null>(null);
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id: otid });
  const [kind, setKind] = useState<ChnotKind>(initialKind ?? ChnotKind.MDWT);
  const [fixedKind, setFixedKind] = useState<boolean>(
    initialKind !== undefined,
  );
  const [fullscreen, setFullscreen] = useState<boolean>(false);

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  const handlePostSave = useCallback(
    async (arg: PostSaveArg) => {
      const meta = {
        otid: otid,
        kind: kind,
        kspace: kspace,
        tid: genTID(),
      };
      let title = "";
      if (arg.title !== titleRef.current) {
        title = arg.title ?? "";
        titleRef.current = title;
        if (kind !== ChnotKind.MDWT) {
          await mdwtCommit({
            mdwt: {
              otid: otid,
              content: title,
            },
          });
        }
      }
      if (
        saveStateRef.current === SaveState.Initial &&
        kind &&
        arg.saveState === SaveState.Saved
      ) {
        await chnotMetaCommit({ metas: [meta] });
        saveStateRef.current = arg.saveState;
      }
      onPostSave(arg);

      setFixedKind(true);
    },
    [kind, kspace, otid],
  );

  const props = useMemo(() => {
    return {
      otid: otid,
      readonly: true,
      fullscreen,
      onPostSave: handlePostSave,
      onSetFullscreen: setFullscreen,
    };
  }, [fullscreen, otid]);

  return (
    <div
      ref={setNodeRef}
      style={style}
      className="flex flex-col w-full"
      {...attributes}
    >
      <div className="flex flex items-center justify-center mt-1 w-full items-center w-full text-gray-400">
        {fixedKind ? (
          <>
            <div
              {...listeners}
              className="p-1 cursor-grab active:cursor-grabbing hover:bg-gray-100 rounded transition-colors"
              title="Drag Handler"
            >
              <ChnotKindIcon kind={kind} className="w-4 h-4" />
            </div>
          </>
        ) : (
          <>
            <div
              {...listeners}
              className="p-1 cursor-grab active:cursor-grabbing hover:bg-gray-100 rounded transition-colors"
              title="Drag Handler"
            >
              <Icon.Hand className="w-4 h-4" />
            </div>
            {Object.values(ChnotKind).map((e) => (
              <ChnotKindIcon
                className="w-4 h-4 mx-1"
                kind={e}
                key={e}
                onClick={() => setKind(e)}
              />
            ))}
          </>
        )}

        <span className="flex-1 border-t border-gray-200 my-2 w-full" />
        {typeof otid === "number" && (
          <span className="text-xs mx-2">{otid}</span>
        )}
        <button
          type="button"
          onClick={() => setFullscreen(true)}
          className="p-1 hover:text-green-600 hover:bg-gray-100 rounded transition-colors"
          title="Edit"
        >
          <Icon.Edit className="w-4 h-4 cursor-pointer" />{" "}
        </button>

        <button
          type="button"
          onClick={() => handleRemoveBlock(otid)}
          className="p-1  hover:text-red-600 hover:bg-gray-100 rounded transition-colors"
          title="Remove"
        >
          <Icon.Trash2 className="w-4 h-4" />
        </button>
        <button
          type="button"
          onClick={() => handleAddBlock(index, true)}
          className="p-1  hover:text-blue-600 hover:bg-gray-100 rounded transition-colors"
          title="Search And Add"
        >
          <Icon.ZoomIn className="w-4 h-4" />
        </button>
        <button
          type="button"
          onClick={() => handleAddBlock(index, false)}
          className="p-1  hover:text-green-600 hover:bg-gray-100 rounded transition-colors"
          title="Add"
        >
          <Icon.Plus className="w-4 h-4" />
        </button>
      </div>
      <div className="w-full ml-4">
        {kind === ChnotKind.ExcalidrawV1 ? (
          <ExcalidrawChnot {...props} />
        ) : kind === ChnotKind.KFileV1 ? (
          <KFileChnot {...props} />
        ) : kind === ChnotKind.KTab ? (
          <TableChnot {...props} readonly={false} />
        ) : kind === ChnotKind.LLMChat ? (
          <LLMChatChnot {...props} />
        ) : kind === ChnotKind.MindMapV1 ? (
          <MindMapChnot {...props} />
        ) : (
          <RichMdwt {...props} readonly={false} />
        )}
      </div>
    </div>
  );
};

const SortableRichMdwtMemo = memo(SortableRichMdwt);

export default SortableRichMdwtMemo;
