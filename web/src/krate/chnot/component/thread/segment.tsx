import { useSortable } from "@dnd-kit/sortable";
import { memo, useCallback, useEffect, useMemo, useRef, useState } from "react";
import Icon from "@/common/component/icon";
import { SaveState } from "@/common/types";
import { mdwtCommit } from "@/krate/mdwt/service";
import { genTID, type TID } from "@/lib/id_util";
import { ChnotKind } from "../../po";
import { chnotMetaCommit } from "../../service";
import { ChnotKindIcon } from "../kind-icon";
import ExcalidrawChnot from "../rich-chnot/excalidraw";
import KFileChnot from "../rich-chnot/kfile";
import LLMChatChnot from "../rich-chnot/llmchat";
import MindMapChnot from "../rich-chnot/mindmap";
import RichMdwt from "../rich-chnot/rich-mdwt";
import type { PostSaveArg, RichPropProps } from "../rich-chnot/rich-mdwt-side";
import TableChnot from "../rich-chnot/table";
import { Agent } from "http";
import { chnotShortDate } from "@/lib/date-utils";
import ReadableTID from "@/common/component/chnot-read-tid";
import { GEN_TITLE } from "@/krate/mdwt/constaints";
import MdwtChnot from "../rich-chnot/mdwt";

const SortableRichChnot = ({
  otid,
  index,
  content,
  closed: initialClosed,
  onPostSave,
  onAddBlock: handleAddBlock,
  onRemoveBlock: handleRemoveBlock,
  onToggleClosed,
  kspace,
  kind: initialKind,
  isDragging: isItemDragging,
  saveState: initialSaveState,
}: {
  otid: TID;
  index: number;
  content: string;
  kspace: string;
  kind?: ChnotKind;
  closed: boolean;
  saveState?: SaveState;
  onAddBlock: (position: number, find: boolean) => void;
  onRemoveBlock: (otid: string | TID) => void;
  onPostSave: (arg: PostSaveArg) => void;
  onToggleClosed: (index: number, closed: boolean) => void;
  isDragging?: boolean;
}) => {
  const saveStateRef = useRef<SaveState>(SaveState.Initial);
  const titleRef = useRef<string | null>(null);
  const { attributes, listeners, setNodeRef, isDragging } = useSortable({
    id: otid,
  });
  const [kind, setKind] = useState<ChnotKind>(initialKind ?? ChnotKind.MDWT);
  const [fullscreen, setFullscreen] = useState<boolean>(false);
  const [closed, setClosed] = useState<boolean>(initialClosed);
  const [saveState, setSaveState] = useState<SaveState | undefined>(
    initialSaveState,
  );

  const style = {
    opacity: isDragging || isItemDragging ? 0.5 : 1,
  };

  useEffect(() => {
    onToggleClosed(index, closed);
  }, [onToggleClosed, closed, index]);

  const handlePostSave = useCallback(
    async (arg: PostSaveArg, manualSaveTitle?: boolean) => {
      const meta = {
        otid: arg.otid,
        kind: arg.kind,
        kspace: kspace,
        tid: genTID(),
      };
      if (manualSaveTitle && arg.title) {
        await mdwtCommit({
          mdwt: {
            otid: arg.otid,
            content: arg.title,
          },
        });
      } else if (
        arg.title &&
        arg.title.length > 0 &&
        arg.title !== titleRef.current &&
        (!titleRef.current || titleRef.current.startsWith(GEN_TITLE))
      ) {
        const title = GEN_TITLE + arg.title;
        if (arg.kind !== ChnotKind.MDWT) {
          await mdwtCommit({
            mdwt: {
              otid: arg.otid,
              content: title,
            },
          });
        }
        titleRef.current = title;
      }
      if (
        saveStateRef.current === SaveState.Initial &&
        arg.saveState === SaveState.Saved
      ) {
        await chnotMetaCommit({ metas: [meta] });
        saveStateRef.current = arg.saveState;
      }
      onPostSave({ ...arg });

      setSaveState(arg.saveState);
    },
    [kspace],
  );

  const props = useMemo(() => {
    return {
      otid: otid,
      readonly: true,
      fullscreen,
      onPostSave: handlePostSave,
      onSetFullscreen: setFullscreen,
      showEditWhenEmpty: true,
    };
  }, [fullscreen, otid]);

  const savePh = useCallback(() => {}, []);

  return (
    <div
      ref={setNodeRef}
      style={style}
      className="flex flex-col w-full"
      {...attributes}
    >
      <div className="flex flex items-center justify-center mt-1 w-full items-center w-full text-gray-400">
        {saveState === SaveState.Saved ? (
          <>
            <div
              {...listeners}
              className="p-1 cursor-grab active:cursor-grabbing hover:bg-gray-100 rounded transition-colors"
              title="Drag Handler"
            >
              <ChnotKindIcon kind={kind} className="w-4 h-4" />
            </div>
            <button
              type="button"
              onClick={() => setClosed((prev) => !prev)}
              className="p-1  hover:text-red-600 hover:bg-gray-100 rounded transition-colors"
              title="Remove"
            >
              {closed ? (
                <Icon.EyeClosed className="w-4 h-4 cursor-pointer" />
              ) : (
                <Icon.Eye className="w-4 h-4 cursor-pointer" />
              )}
            </button>
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
                className="w-4 h-4 mx-1 hover:cursor-pointer"
                kind={e}
                key={e}
                onClick={() => setKind(e)}
              />
            ))}
          </>
        )}
        <button
          type="button"
          onClick={() => setFullscreen(true)}
          className="p-1 hover:text-green-600 hover:bg-gray-100 rounded transition-colors"
          title="Edit"
        >
          <Icon.Edit className="w-4 h-4 cursor-pointer" />
        </button>
        {saveState && (
          <span className="p-1 rounded">
            {saveState === SaveState.Saved ? (
              <Icon.CloudCheck className="w-4 h-4" />
            ) : saveState === SaveState.Dirty ? (
              <Icon.CloudDrizzle className="w-4 h-4" />
            ) : (
              <Icon.CloudAlert className="w-4 h-4" />
            )}
          </span>
        )}

        <span className="flex-1 border-t border-gray-200 my-2 w-full" />

        {typeof otid === "number" && <ReadableTID tid={otid} />}

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
          <Icon.LinkIcon className="w-4 h-4" />
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
      {kind !== ChnotKind.MDWT && (
        <div className="w-full ml-4">
          <MdwtChnot
            otid={props.otid}
            fullscreen={false}
            onPostSave={(arg) => {
              handlePostSave({ ...arg, kind: kind }, true);
            }}
            content={content.replace(GEN_TITLE, "")}
          />
        </div>
      )}
      {closed || (
        <div className="w-full ml-4">
          <RichChnotMemo props={props} kind={kind} />
        </div>
      )}
    </div>
  );
};

const RichChnotMemo = memo(
  ({
    kind,
    props,
  }: {
    kind?: ChnotKind;
    props: RichPropProps & { showEditWhenEmpty: boolean };
  }) => {
    return kind === ChnotKind.ExcalidrawV1 ? (
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
    );
  },
);

export default SortableRichChnot;
