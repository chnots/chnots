import { genTID, TID } from "@/lib/id_util";
import { ChnotKind, ChnotMeta } from "../../po";
import RichMdwt from "../rich-chnot/rich-mdwt";
import ExcalidrawChnot from "../rich-chnot/excalidraw";
import KFileChnot from "../rich-chnot/kfile";
import LLMChatChnot from "../rich-chnot/llmchat";
import TableChnot from "../rich-chnot/table";
import { memo, useCallback, useRef } from "react";
import { PostSaveArg } from "../rich-chnot/rich-chnot";
import { SaveState } from "@/common/types";
import { chnotMetaCommit } from "../../service";
import { useKSpaceStore } from "@/krate/kspace/store";
import { useChnotSingleStore } from "../../store";
import React from "react";

const ChnotSingleBody = ({
  otid,
  kind,
  setKind,
}: {
  otid: TID;
  kind: ChnotKind;
  setKind: (kind: ChnotKind) => void;
}) => {
  const saveStateRef = useRef<SaveState>(SaveState.Initial);
  const titleRef = useRef<string | null>(null);
  const initialed = useRef<boolean>(false);

  const { kspace } = useKSpaceStore((s) => {
    return {
      kspace: s.currentKSpace,
    };
  });
  const metaRef = useRef<ChnotMeta>({
    otid: otid,
    kind: kind,
    kspace: kspace,
    tid: genTID(),
  });

  const { overwrite, setCurOtid } = useChnotSingleStore((s) => {
    return {
      overwrite: s.overwrite,
      setCurOtid: s.setCurOtid,
    };
  });

  const handlePostSave = useCallback(
    async (arg: PostSaveArg) => {
      if (
        saveStateRef.current === SaveState.Initial &&
        kind &&
        arg.saveState === SaveState.Saved
      ) {
        await chnotMetaCommit({ metas: [metaRef.current] });
      }
      overwrite({
        meta: metaRef.current,
        title: titleRef.current ?? undefined,
      });
      setCurOtid(otid);
      saveStateRef.current = arg.saveState;
    },
    [kind],
  );

  const props = {
    otid: otid,
    readonly: false,
    fullscreen: false,
    onPostSave: (arg: PostSaveArg) => {
      handlePostSave(arg);
    },
  };

  return (
    <div className="flex w-full h-full justify-center">
      {kind === ChnotKind.ExcalidrawV1 ? (
        <ExcalidrawChnot {...props} />
      ) : kind === ChnotKind.KFileV1 ? (
        <KFileChnot {...props} />
      ) : kind == ChnotKind.KTab ? (
        <TableChnot {...props} />
      ) : kind === ChnotKind.LLMChat ? (
        <LLMChatChnot {...props} />
      ) : (
        <RichMdwt
          {...props}
          tryfetch={true}
          onChanged={(cont: string) => {
            if (!initialed.current) {
              setKind(ChnotKind.MDWT);
              initialed.current = true;
            }
            const title = cont.length > 100 ? cont.substring(0, 100) : cont;
            if (title !== titleRef.current) {
              titleRef.current = title;
            }
          }}
          whfull={true}
        />
      )}
    </div>
  );
};

export const ChnotSingleBodyMemo = React.memo(ChnotSingleBody);

export default ChnotSingleBody;
