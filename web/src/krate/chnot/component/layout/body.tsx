import React, {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { SaveState } from "@/common/types";
import { useKSpaceStore } from "@/krate/kspace/store";
import { GEN_TITLE } from "@/krate/mdwt/constaints";
import { mdwtCommit } from "@/krate/mdwt/service";
import { genTID, type TID } from "@/lib/id_util";
import { ChnotKind } from "../../po";
import { chnotMetaCommit } from "../../service";
import { useChnotStore } from "../../store";
import ExcalidrawChnot from "../rich-chnot/excalidraw";
import KFileChnot from "../rich-chnot/kfile";
import LLMChatChnot from "../rich-chnot/llmchat";
import MindMapChnot from "../rich-chnot/mindmap";
import RichMdwt from "../rich-chnot/rich-mdwt";
import type { PostSaveArg } from "../rich-chnot/rich-mdwt-side";
import TableChnot from "../rich-chnot/table";
import { ChnotThreadMemo } from "../rich-chnot/thread";

const ChnotBody = ({ otid, kind }: { otid: TID; kind: ChnotKind }) => {
  const saveStateRef = useRef<SaveState>(SaveState.Initial);
  const titleRef = useRef<string | null>(null);

  const { kspace } = useKSpaceStore((s) => {
    return {
      kspace: s.currentKSpace,
    };
  });

  const { overwrite, setCurOtid, getMeta } = useChnotStore((s) => {
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
      const meta = {
        otid: arg.otid,
        kind: arg.kind,
        kspace: kspace,
        tid: genTID(),
      };
      let title: string | undefined;
      if (
        arg.title &&
        arg.title.length > 0 &&
        arg.title !== titleRef.current &&
        (!titleRef.current || titleRef.current.startsWith(GEN_TITLE))
      ) {
        title = GEN_TITLE + arg.title;
        if (arg.kind !== ChnotKind.MDWT) {
          await mdwtCommit({
            mdwt: {
              otid: arg.otid,
              content: title,
            },
          });
        }
        overwrite({
          meta: meta,
          title: title,
        });
        titleRef.current = title;
      }
      if (
        saveStateRef.current === SaveState.Initial &&
        arg.kind &&
        arg.saveState === SaveState.Saved
      ) {
        await chnotMetaCommit({ metas: [meta] });
        saveStateRef.current = arg.saveState;
        setCurOtid(otid);
      }
    },
    [kspace, overwrite, setCurOtid],
  );

  const props = useMemo(() => {
    return {
      otid: otid,
      readonly: false,
      fullscreen: false,
      onPostSave: handlePostSave,
      showEditWhenEmpty: false,
    };
  }, [otid]);

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
  ) : kind === ChnotKind.ThreadV1 ? (
    <div className="w-full h-full">
      <ChnotThreadMemo {...props} />
    </div>
  ) : (
    <div className="flex flex-col w-full items-center m-0 p-1 h-full">
      <RichMdwt {...props} />
    </div>
  );
};

export const ChnotBodyMemo = React.memo(ChnotBody);

export default ChnotBody;
