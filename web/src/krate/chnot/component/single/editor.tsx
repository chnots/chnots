import { TID } from "@/lib/id_util";
import { useChnotSingleStore } from "../../store";
import { useEffect, useState } from "react";
import { ChnotSearchRspSingle } from "../../dto";
import { ChnotKind } from "../../po";
import { PostSaveArg } from "../rich-chnot/rich-chnot";
import RichMdwt from "../rich-chnot/rich-mdwt";
import ExcalidrawBlock from "../rich-chnot/excalidraw";
import KFileBlock from "../rich-chnot/kfile";
import LLMChatChnot from "../rich-chnot/llmchat";
import TableChnot from "../rich-chnot/table";

const ChnotSingleEditor = ({ otid }: { otid: TID }) => {
  const { getMeta } = useChnotSingleStore((s) => {
    return {
      getMeta: s.getMeta,
    };
  });

  const [meta, setMeta] = useState<ChnotSearchRspSingle>();
  useEffect(() => {
    setMeta(getMeta(otid));
  }, []);

  const kind = meta?.meta.kind;

  const props = {
    otid: otid,
    readonly: false,
    fullscreen: true,
    onPostSave: (arg: PostSaveArg) => {},
    onSetFullscreen: (flag: boolean) => {},
  };

  return kind === ChnotKind.ExcalidrawV1 ? (
    <ExcalidrawBlock {...props} />
  ) : kind === ChnotKind.KFileV1 ? (
    <KFileBlock {...props} />
  ) : kind == ChnotKind.KTab ? (
    <TableChnot {...props} />
  ) : kind === ChnotKind.LLMChat ? (
    <LLMChatChnot {...props} />
  ) : (
    <RichMdwt
      otid={0}
      onPostSave={function (arg: PostSaveArg): void {}}
      onChanged={function (): void {}}
    />
  );
};

export default ChnotSingleEditor;
