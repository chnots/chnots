import React, { useCallback, useEffect, useRef, useState } from "react";
import { SaveState } from "@/common/types";
import { useKSpaceStore } from "@/krate/kspace/store";
import { mdwtCommit } from "@/krate/mdwt/service";
import { genTID, type TID } from "@/lib/id_util";
import { ChnotKind } from "../../po";
import { chnotMetaCommit } from "../../service";
import { useChnotSingleStore } from "../../store";
import ExcalidrawChnot from "../rich-chnot/excalidraw";
import KFileChnot from "../rich-chnot/kfile";
import LLMChatChnot from "../rich-chnot/llmchat";
import type { PostSaveArg } from "../rich-chnot/rich-chnot";
import RichMdwt from "../rich-chnot/rich-mdwt";
import TableChnot from "../rich-chnot/table";
import MindMapChnot from "../rich-chnot/mindmap";
import MdwtChnot from "../rich-chnot/mdwt";

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
  }, [getMeta, otid]);

  const handlePostSave = useCallback(
    async (arg: PostSaveArg) => {
      console.log("save kind", kind);
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
        overwrite({
          meta: meta,
          title: titleRef.current ?? undefined,
        });
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
    [kind, kspace, otid, overwrite, setCurOtid],
  );

  const handleChange = useCallback(() => { }, []);

  const props = {
    otid: otid,
    readonly: false,
    fullscreen: false,
    onPostSave: handlePostSave,
  };


  console.log("kind: ", kind);
  return kind === ChnotKind.ExcalidrawV1 ? (
    <div className="flex w-full h-full overflow-auto">
      <ExcalidrawChnot {...props} />
    </div>
  ) : kind === ChnotKind.KFileV1 ? (
    <KFileChnot {...props} />
  ) : kind === ChnotKind.KTab ? (
    <div className="w-full h-full">
      <TableChnot {...props} />
    </div>
  ) : kind === ChnotKind.LLMChat ? (
    <div className="flex w-full h-full overflow-auto">
      <LLMChatChnot {...props} />
    </div>
  ) : kind === ChnotKind.MindMapV1 ? (
    <div className="flex flex-col w-full items-center m-0 p-1 h-full">
      <MindMapChnot {...props} />
    </div>
  ) : (
    <div className="flex flex-col w-full items-center m-0 p-1 h-full">
      <RichMdwt {...props} />
    </div>
  );
};

export const ChnotSingleBodyMemo = React.memo(ChnotSingleBody);

export default ChnotSingleBody;
