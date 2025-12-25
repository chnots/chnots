import type { KfileMetaFetchReqId } from "@/krate/kfile/dto";
import { kfileInlineDownload, kfileInlineUpload } from "@/krate/kfile/service";
import { genTID, genUID, type TID } from "@/lib/id_util";
import type { MindElixirData } from "mind-elixir";

export type MindElixirChnotData = {
  otid: TID;
  data: MindElixirData
};

export const fetchMindExilir = async (
  id: KfileMetaFetchReqId,
): Promise<MindElixirData | null> => {
  try {
    const rsp = await kfileInlineDownload({
      req_id: id,
    });
    if (!rsp.file) {
      return null;
    }
    const data = JSON.parse(rsp.file?.content);
    return data;
  } catch (_e) { }
  return null;
};

export type SaveMindExilirProps = {
  otid: TID,
  data: MindElixirData;
  onSuccess: () => void;
  onFail: () => void;
};

export const saveMindExilir = async (props: SaveMindExilirProps) => {
  const { otid, data, onSuccess, onFail } = props;

  try {
    await kfileInlineUpload({
      res: {
        tid: genTID(),
        content: JSON.stringify(data),
        sid: "placeholder",
      },
      meta_id: genUID(),
      archor_intervals: 3600,
      content_type: "mind-elixir-v5",
      otid: otid,
    });
    onSuccess();
  } catch (_err) {
    onFail();
  }
};
