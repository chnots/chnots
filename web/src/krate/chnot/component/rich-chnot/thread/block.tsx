import { useSortable } from "@dnd-kit/sortable";
import {
  CloudAlert,
  CloudCheck,
  CloudDrizzle,
  Edit,
  Eye,
  EyeClosed,
  Hand,
  LinkIcon,
  Plus,
  Trash2,
} from "lucide-react";
import { memo, useCallback, useEffect, useMemo, useRef, useState } from "react";
import ReadableTID from "@/common/component/chnot-read-tid";
import { SaveState } from "@/common/types";
import { GEN_TITLE } from "@/krate/mdwt/constaints";
import { mdwtCommit } from "@/krate/mdwt/service";
import { genTID, type TID } from "@/lib/id_util";
import { ChnotKind } from "../../../po";
import { chnotMetaCommit } from "../../../service";
import { ChnotKindIcon } from "../../kind-icon";
import ExcalidrawChnot from "../excalidraw";
import KFileChnot from "../kfile";
import LLMChatChnot from "../llmchat";
import MdwtChnot from "../mdwt";
import MindMapChnot from "../mindmap";
import RichMdwt from "../rich-mdwt";
import type { PostSaveArg, RichPropProps } from "../rich-mdwt-side";
import TableChnot from "../table";

const SortableRichBlock = ({
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
  onPostSave: (arg: PostSaveArg) => Promise<void>;
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

  const handleToggleCloses = useCallback(
    (closed: boolean) => {
      setClosed(closed);
      onToggleClosed(index, closed);
    },
    [onToggleClosed, index],
  );

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
      disableHeaderActions: true,
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
              onClick={() => handleToggleCloses(!closed)}
              className="p-1  hover:text-red-600 hover:bg-gray-100 rounded transition-colors"
              title="Remove"
            >
              {closed ? (
                <EyeClosed className="w-4 h-4 cursor-pointer" />
              ) : (
                <Eye className="w-4 h-4 cursor-pointer" />
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
              <Hand className="w-4 h-4" />
            </div>
            <div className="flex border p-0.5 rounded text-gray-600">
              {Object.values(ChnotKind).map((e) =>
                e !== ChnotKind.ThreadV1 ? (
                  <ChnotKindIcon
                    className="w-4 h-4 mx-1 hover:cursor-pointer"
                    kind={e}
                    key={e}
                    onClick={() => setKind(e)}
                  />
                ) : undefined,
              )}
            </div>
          </>
        )}
        <button
          type="button"
          onClick={() => setFullscreen(true)}
          className="p-1 hover:text-green-600 hover:bg-gray-100 rounded transition-colors"
          title="Edit"
        >
          <Edit className="w-4 h-4 cursor-pointer" />
        </button>
        {!!saveState && (
          <span className="p-1 rounded">
            {saveState === SaveState.Saved ? (
              <CloudCheck className="w-4 h-4" />
            ) : saveState === SaveState.Dirty ? (
              <CloudDrizzle className="w-4 h-4" />
            ) : (
              <CloudAlert className="w-4 h-4" />
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
          <Trash2 className="w-4 h-4" />
        </button>
        <button
          type="button"
          onClick={() => handleAddBlock(index, true)}
          className="p-1  hover:text-blue-600 hover:bg-gray-100 rounded transition-colors"
          title="Search And Add"
        >
          <LinkIcon className="w-4 h-4" />
        </button>
        <button
          type="button"
          onClick={() => handleAddBlock(index, false)}
          className="p-1  hover:text-green-600 hover:bg-gray-100 rounded transition-colors"
          title="Add"
        >
          <Plus className="w-4 h-4" />
        </button>
      </div>
      {kind !== ChnotKind.MDWT && (
        <div className="w-full ml-4">
          <MdwtChnot
            otid={props.otid}
            fullscreen={false}
            onPostSave={async (arg) => {
              await handlePostSave({ ...arg, kind: kind }, true);
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

export default SortableRichBlock;
