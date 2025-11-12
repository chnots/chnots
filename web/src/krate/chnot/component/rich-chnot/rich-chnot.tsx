import { useCallback, useState } from "react";
import { ChnotKind } from "../../po";
import { SaveState } from "@/common/types";
import MdwtChnot from "./mdwt";
import ExcalidrawChnot from "./excalidraw";
import { ChnotKindIcon } from "../kind-icon";
import KFileChnot from "./kfile";
import TableChnot from "./table";
import LLMChatChnot from "./llmchat";
import { chnotMetaCommit } from "@/krate/chnot/service";
import { genTID, TID } from "@/lib/id_util";
import { Button } from "@/common/component/ui/button";
import Icon from "@/common/component/icon";
import { cachedChnotMapByOtid } from "../../store";

export type PostSaveArg = {
  saveState: SaveState;
  title: string;
};

export type RichPropProps = {
  otid: TID;
  readonly?: boolean;
  fullscreen: boolean;
  onSetFullscreen?: (flag: boolean) => void;
  onPostSave: (arg: PostSaveArg) => void;
};

const ChnotKindSelector = ({
  setKind,
}: {
  setKind: (kind: ChnotKind) => void;
}) => {
  return Object.values(ChnotKind).map((kind) => {
    return (
      <Button onClick={() => setKind(kind)}>
        <ChnotKindIcon kind={kind} />
      </Button>
    );
  });
};

const RichChnot = ({
  otid,
  readonly,
  kspace,
}: {
  otid: TID;
  readonly?: boolean;
  kspace: string;
}) => {
  console.log("render RichChnot", otid);

  const [fullscreen, setFullscreen] = useState<boolean>(false);
  const [saveState, setSaveState] = useState(SaveState.Initial);
  const [kind, setKind] = useState<ChnotKind | undefined>(
    cachedChnotMapByOtid.get(otid)?.kind,
  );

  const handlePostSave = useCallback(
    async (arg: PostSaveArg) => {
      if (
        saveState === SaveState.Initial &&
        kind &&
        arg.saveState === SaveState.Saved
      ) {
        const meta = {
          otid: otid,
          kind: kind,
          kspace: kspace,
          tid: genTID(),
        };

        await chnotMetaCommit({ metas: [meta] });

        cachedChnotMapByOtid.set(meta.otid, meta);
      }
      setSaveState(arg.saveState);
    },
    [saveState, kind],
  );

  const props = {
    otid: otid,
    readonly: readonly,
    fullscreen,
    onPostSave: (arg: PostSaveArg) => {
      handlePostSave(arg);
    },
    onSetFullscreen: (flag: boolean) => {
      setFullscreen(flag);
    },
  };

  return (
    <div className="relative flex items-start space-x-2 px-2 py-0 my-1 rounded bg-white">
      <Button
        onClick={() => setFullscreen(true)}
        className="absolute top-1 right-1 z-49"
      >
        <Icon.Fullscreen />
      </Button>
      <div
        className="flex-1 rounded focus:outline-none h-full space-y-2 border max-w-full p-1"
        tabIndex={0}
        aria-label="Text block, click to edit"
      >
        {kind === ChnotKind.MDWT ? (
          <MdwtChnot {...props} tryFetch={true} />
        ) : kind === ChnotKind.ExcalidrawV1 ? (
          <ExcalidrawChnot {...props} />
        ) : kind === ChnotKind.KFileV1 ? (
          <KFileChnot {...props} />
        ) : kind == ChnotKind.KTab ? (
          <TableChnot {...props} />
        ) : kind === ChnotKind.LLMChat ? (
          <LLMChatChnot {...props} />
        ) : (
          <ChnotKindSelector
            setKind={function (kind: ChnotKind): void {
              setKind(kind);
            }}
          />
        )}
      </div>
    </div>
  );
};

export default RichChnot;
