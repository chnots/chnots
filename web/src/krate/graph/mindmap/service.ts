import type { KfileMetaFetchReqId } from "@/krate/kfile/dto";
import { kfileInlineDownload, kfileInlineUpload } from "@/krate/kfile/service";
import { genTID, type TID } from "@/lib/id_util";
import type { MindElixirData } from "mind-elixir-react";

export type MindElixirChnotState = {
  otid: TID;
  metaId: string;
  data: MindElixirData
};

export const fetchMindExilir = async (
  id: KfileMetaFetchReqId,
): Promise<MindElixirChnotState | null> => {
  try {
    const rsp = await kfileInlineDownload({
      req_id: id,
    });
    if (!rsp.file) {
      return null;
    }
    const data = JSON.parse(rsp.file?.content);
    return {
      data,
      otid: rsp.meta!.otid,
      metaId: rsp.meta!.id,
    };
  } catch (_e) { }
  return null;
};

export type SaveMindExilirProps = {
  state: MindElixirChnotState;
  contentType: string;
  onSuccess: () => void;
  onFail: () => void;
};

export const saveMindExilir = async (props: SaveMindExilirProps) => {
  const { state, contentType, onSuccess, onFail } = props;

  try {
    await kfileInlineUpload({
      res: {
        tid: genTID(),
        content: JSON.stringify(state.data),
        sid: "placeholder",
      },
      archor_intervals: 3600,
      meta_id: state.metaId,
      content_type: contentType,
      otid: state.otid,
    });
    onSuccess();
  } catch (_err) {
    onFail();
  }
};
