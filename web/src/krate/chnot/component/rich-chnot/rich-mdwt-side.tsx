import { useCallback, useState } from "react";
import Icon from "@/common/component/icon";
import { Button } from "@/common/component/ui/button";
import { SaveState } from "@/common/types";
import { chnotMetaCommit } from "@/krate/chnot/service";
import { chnotShortDate } from "@/lib/date-utils";
import { genTID, type TID } from "@/lib/id_util";
import { ChnotKind } from "../../po";
import { cachedChnotMapByOtid } from "../../store";
import { ChnotKindIcon } from "../kind-icon";
import ExcalidrawChnot from "./excalidraw";
import KFileChnot from "./kfile";
import LLMChatChnot from "./llmchat";
import MdwtChnot from "./mdwt";
import TableChnot from "./table";
import MindMapChnot from "./mindmap";

export type PostSaveArg = {
  otid: TID;
  saveState: SaveState;
  kind: ChnotKind;
  title?: string;
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
      <Button onClick={() => setKind(kind)} key={kind}>
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
    [saveState, kind, kspace, otid],
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
    <div className="flex flex-col items-start px-2 py-0 my-1 rounded bg-white min-h-12 w-full">
      <div className="flex items-center justify-between border-y border-muted h-5 w-full text-muted-foreground">
        <div className="flex items-center space-x-2">
          {kind && <ChnotKindIcon kind={kind} className="w-4 h-4" />}
          <time
            dateTime={new Date(otid / 1e3).toISOString()}
            className="text-[0.7rem] whitespace-nowrap"
          >
            {chnotShortDate(new Date(otid / 1e3))}
          </time>
        </div>
        <Icon.Fullscreen
          className="w-4 h-4 cursor-pointer"
          onClick={() => setFullscreen(true)}
        />
      </div>
      <div className="flex-1 focus:outline-none h-full space-y-2 max-w-full p-1 w-full">
        {kind === ChnotKind.MDWT ? (
          <MdwtChnot {...props} />
        ) : kind === ChnotKind.ExcalidrawV1 ? (
          <ExcalidrawChnot {...props} readonly={true} />
        ) : kind === ChnotKind.KFileV1 ? (
          <KFileChnot {...props} />
        ) : kind === ChnotKind.KTab ? (
          <TableChnot {...props} />
        ) : kind === ChnotKind.LLMChat ? (
          <LLMChatChnot {...props} />
        ) : kind === ChnotKind.MindMapV1 ? (
          <MindMapChnot {...props} readonly={true} />
        ) : (
          <ChnotKindSelector
            setKind={(kind: ChnotKind): void => {
              setKind(kind);
            }}
          />
        )}
      </div>
    </div>
  );
};

export default RichChnot;
