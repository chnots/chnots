import { genTID, TID } from "@/lib/id_util";
import { ChnotKind, ChnotMeta } from "../../po";
import RichMdwt from "../rich-chnot/rich-mdwt";
import ExcalidrawChnot from "../rich-chnot/excalidraw";
import KFileChnot from "../rich-chnot/kfile";
import LLMChatChnot from "../rich-chnot/llmchat";
import TableChnot from "../rich-chnot/table";
import { memo, useCallback, useEffect, useRef } from "react";
import { PostSaveArg } from "../rich-chnot/rich-chnot";
import { SaveState } from "@/common/types";
import { chnotMetaCommit } from "../../service";
import { useKSpaceStore } from "@/krate/kspace/store";
import { useChnotSingleStore } from "../../store";
import React from "react";
import { mdwtCommit } from "@/krate/mdwt/service";

const ChnotSingleBody = ({ otid, kind }: { otid: TID; kind: ChnotKind }) => {
  const saveStateRef = useRef<SaveState>(SaveState.Initial);
  const titleRef = useRef<string | null>(null);

  const { kspace } = useKSpaceStore((s) => {
    return {
      kspace: s.currentKSpace,
    };
  });

  const { overwrite, setCurOtid, getMeta } = useChnotSingleStore((s) => {
    return {
      overwrite: s.overwrite,
      setCurOtid: s.setCurOtid,
      getMeta: s.getMeta,
    };
  });

  useEffect(() => {
    if (getMeta(otid)) {
      saveStateRef.current = SaveState.Saved;
    }
  }, []);

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

  return kind === ChnotKind.ExcalidrawV1 ? (
    <div className="flex w-full h-full overflow-auto">
      <ExcalidrawChnot {...props} />
    </div>
  ) : kind === ChnotKind.KFileV1 ? (
    <KFileChnot {...props} />
  ) : kind == ChnotKind.KTab ? (
    <div className="w-full h-full">
      <TableChnot {...props} />
    </div>
  ) : kind === ChnotKind.LLMChat ? (
    <div className="flex w-full h-full overflow-auto">
      <LLMChatChnot {...props} />
    </div>
  ) : (
    <div className="flex w-full items-center justify-center m-0 p-1 border-2 border-blue-300">
      <RichMdwt
        {...props}
        tryfetch={true}
        onPostSave={(arg) => {
          handlePostSave(arg);
        }}
        onChanged={() => {}}
      />
    </div>
  );
};

export const ChnotSingleBodyMemo = React.memo(ChnotSingleBody);

export default ChnotSingleBody;
