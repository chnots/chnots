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
import { useKSpaceStore } from "@/krate/kspace/store";
import { TID } from "@/lib/id_util";

export type PostSaveArg = {
  saveState: SaveState;
};

export type ChnotChromeProps = {
  otid: TID;
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
  const kind = meta.kind;
  const [saveState, setSaveState] = useState(SaveState.Initial);
  const { kspace } = useKSpaceStore((s) => {
    return {
      kspace: s.currentKSpace,
    };
  });

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
              kspace: kspace,
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
            otid={meta.chnotOtid}
            readonly={readonly}
            onPostSave={(arg: PostSaveArg) => {
              handlePostSave(arg);
            }}
          />
        ) : kind === ChnotKind.ExcalidrawV1 ? (
          <ExcalidrawBlock
            readonly={readonly}
            otid={meta.chnotOtid}
            onPostSave={(arg: PostSaveArg) => {
              handlePostSave(arg);
            }}
          />
        ) : kind === ChnotKind.KFileV1 ? (
          <KFileBlock
            otid={meta.chnotOtid}
            onPostSave={(arg: PostSaveArg) => {
              handlePostSave(arg);
            }}
          />
        ) : kind == ChnotKind.KTab ? (
          <TableChnot
            otid={meta.chnotOtid}
            onPostSave={function (arg: PostSaveArg): void {
              handlePostSave(arg);
            }}
          />
        ) : kind === ChnotKind.LLMChat ? (
          <LLMChatChnot
            otid={meta.chnotOtid}
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
