import { useCallback, useState } from "react";
import { ChnotKind } from "../../../po";
import { SaveState } from "@/common/types";
import BlockState from "../save-state";
import MdwtRecord from "./mdwt";
import { ChnotMetaKind } from "../../vo";
import ExcalidrawBlock from "./excalidraw";
import { ChnotKindIcon } from "../../chnot-kind-icon";
import KFileBlock from "./kfile";
import TableChnot from "./table";
import LLMChatChnot from "./llmchat";
import { chnotMetaCommit } from "@/krate/chnot/service";

export type PostSaveArg = {
  saveState: SaveState;
  kindId: string;
};

export type ChnotChromeProps = {
  kindId?: string;
  readonly?: boolean;
  onPostSave: (arg: PostSaveArg) => void;
};

const RichChnot = ({
  meta,
  readonly,
}: {
  meta: ChnotMetaKind;
  readonly?: boolean;
}) => {
  const [saveState, setSaveState] = useState(
    meta.kindId ? SaveState.Saved : SaveState.Initial,
  );
  const [kind, setKind] = useState<ChnotKind | undefined>(meta?.kind);

  const handlePostSave = useCallback(
    (arg: PostSaveArg) => {
      if (
        saveState === SaveState.Initial &&
        kind &&
        arg.saveState === SaveState.Saved
      ) {
        chnotMetaCommit({
          metas: [
            {
              otid: meta.chnotOtid,
              kind: kind,
              kind_id: arg.kindId,
            },
          ],
        });
      }
      setSaveState(arg.saveState);
    },
    [saveState, kind],
  );

  return (
    <div className="flex items-start space-x-2 px-2 py-0 my-1 rounded bg-white hover:bg-accent">
      <div className="flex flex-col space-y-1">
        {saveState !== SaveState.Saved || !kind ? (
          <BlockState saveState={saveState} />
        ) : (
          <ChnotKindIcon kind={kind} className="w-4 h-4 text-gray-400 m-1" />
        )}
      </div>

      <div
        className="flex-1 rounded focus:outline-none h-full space-y-2 border"
        tabIndex={0}
        aria-label="Text block, click to edit"
      >
        {kind === ChnotKind.MDWT ? (
          <MdwtRecord
            kindId={meta?.kindId}
            readonly={readonly}
            onPostSave={(arg: PostSaveArg) => {
              handlePostSave(arg);
            }}
          />
        ) : kind === ChnotKind.ExcalidrawV1 ? (
          <ExcalidrawBlock
            readonly={readonly}
            kindId={meta?.kindId}
            onPostSave={(arg: PostSaveArg) => {
              handlePostSave(arg);
            }}
          />
        ) : kind === ChnotKind.KFileV1 ? (
          <KFileBlock
            kindId={meta?.kindId}
            onPostSave={(arg: PostSaveArg) => {
              handlePostSave(arg);
            }}
          />
        ) : kind == ChnotKind.KTab ? (
          <TableChnot
            kindId={meta?.kindId}
            onPostSave={function (arg: PostSaveArg): void {
              handlePostSave(arg);
            }}
          />
        ) : kind === ChnotKind.LLMChat ? (
          <LLMChatChnot
            kindId={meta?.kindId}
            onPostSave={function (arg: PostSaveArg): void {
              handlePostSave(arg);
            }}
          />
        ) : (
          <></>
        )}
      </div>
    </div>
  );
};

export default RichChnot;
