import React, { RefObject, useCallback, useEffect, useState } from "react";
import { ChnotKind } from "../../../po";
import { SaveState } from "@/common/types";
import BlockState from "../chnot-save-state";
import MdwtRecord from "./mdwt";
import { ChnotMetaKind } from "../../vo";
import Tier from "./tier";
import { TID } from "@/lib/id_util";
import ExcalidrawBlock from "./excalidraw";
import { ChnotKindIcon } from "../../chnot-kind-icon";
import KFileBlock from "./kfile";

export type PostSaveArg =
  | {
      kind: ChnotKind.MDWT;
      otid: TID;
      content: string;
      saveState: SaveState;
    }
  | {
      saveState: SaveState;
    };

const Chrome = ({
  otid,
  onMoveUp,
  onMoveDown,
  onDelete,
  onPostSave,
  isFirst,
  isLast,
  blockKindsRef,
}: {
  onMoveUp: () => void;
  onMoveDown: () => void;
  onDelete: () => void;
  onPostSave: (arg: PostSaveArg) => void;
  isFirst: boolean;
  isLast: boolean;
  otid: TID;
  blockKindsRef: RefObject<Map<TID, ChnotMetaKind>>;
}) => {
  const [isFocused, setIsFocused] = useState(false);
  const meta = blockKindsRef.current.get(otid);
  const [saveState, setSaveState] = useState(
    meta?.kind_id ? SaveState.Saved : SaveState.Initial,
  );
  const [kind, setKind] = useState<ChnotKind | undefined>(meta?.kind);

  const handleFocus = () => {
    setIsFocused(true);
  };

  const handleBlur = () => {
    setIsFocused(false);
  };

  const handleKeyDown = (event: React.KeyboardEvent) => {
    if (event.key === "Escape") {
      event.preventDefault();
      handleBlur();
    }
  };

  const handlePostSave = useCallback((arg: PostSaveArg) => {
    setSaveState(arg.saveState);
    onPostSave(arg);
  }, []);

  return (
    <div className="flex items-start space-x-2 px-2 py-0 my-1 rounded-lg bg-white">
      <div className="flex flex-col space-y-1">
        {saveState !== SaveState.Saved || !kind ? (
          <BlockState saveState={saveState} />
        ) : (
          <ChnotKindIcon kind={kind} className="w-4 h-4 text-gray-400 m-1" />
        )}
      </div>

      <div
        className="flex-1 rounded focus:outline-none h-full space-y-2"
        tabIndex={0}
        onFocus={handleFocus}
        onBlur={handleBlur}
        onKeyDown={handleKeyDown}
        aria-label="Text block, click to edit"
      >
        {kind === ChnotKind.MDWT ? (
          <MdwtRecord
            otid={otid}
            isFocused={isFocused}
            onPostSave={(arg: PostSaveArg) => {
              handlePostSave(arg);
            }}
            blockKindsRef={blockKindsRef}
          />
        ) : kind === ChnotKind.ExcalidrawV1 ? (
          <ExcalidrawBlock
            otid={otid}
            isFocused={isFocused}
            kindId={meta?.kind_id}
            onPostSave={(arg: PostSaveArg) => {
              handlePostSave(arg);
            }}
            blockKindsRef={blockKindsRef}
          />
        ) : kind === ChnotKind.KFileV1 ? (
          <KFileBlock
            otid={otid}
            onPostSave={(arg: PostSaveArg) => {
              handlePostSave(arg);
            }}
            blockKindsRef={blockKindsRef}
          />
        ) : (
          <></>
        )}
        {saveState === SaveState.Initial && (
          <Tier
            setKind={function (kind?: ChnotKind): void {
              setKind(kind);
            }}
            hidden={false}
          />
        )}
      </div>
    </div>
  );
};

export default Chrome;
