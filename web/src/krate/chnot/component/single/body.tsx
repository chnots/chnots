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
import { mdwtCommit } from "@/krate/mdwt/service";

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

  const { overwrite, setCurOtid } = useChnotSingleStore((s) => {
    return {
      overwrite: s.overwrite,
      setCurOtid: s.setCurOtid,
    };
  });

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
        overwrite({
          meta: meta,
          title: titleRef.current ?? undefined,
        });
        setCurOtid(otid);
      }
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
    <div className="flex flex-grow w-full items-center justify-center m-0 p-1 border-2 border-blue-300 overflow-hidden">
      {kind === ChnotKind.ExcalidrawV1 ? (
        <div className="flex w-full h-full overflow-auto">
          <ExcalidrawChnot {...props} />
        </div>
      ) : kind === ChnotKind.KFileV1 ? (
        <KFileChnot {...props} />
      ) : kind == ChnotKind.KTab ? (
        <TableChnot {...props} />
      ) : kind === ChnotKind.LLMChat ? (
        <LLMChatChnot {...props} />
      ) : (
        <RichMdwt
          fixedHeight={true}
          {...props}
          tryfetch={true}
          onPostSave={(arg) => {
            handlePostSave(arg);
          }}
          onChanged={(cont: string) => {
            if (!initialed.current) {
              setKind(ChnotKind.MDWT);
              initialed.current = true;
            }
          }}
          whfull={"h-[90%]"}
        />
      )}
    </div>
  );
};

export const ChnotSingleBodyMemo = React.memo(ChnotSingleBody);

export default ChnotSingleBody;
