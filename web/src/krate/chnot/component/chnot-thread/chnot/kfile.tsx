import { SaveState } from "@/common/types";
import { ChnotKind } from "@/krate/chnot/po";
import { CommonKFile } from "@/krate/kfile/components/common-kfile";
import { ChnotChromeProps } from "./chrome";

const KFileBlock = ({ otid, kindId, onPostSave }: ChnotChromeProps) => {
  return (
    <CommonKFile
      kid={kindId}
      onPostSave={(r) => {
        onPostSave({
          saveState: SaveState.Saved,
          data: {
            otid: otid,
            kind: ChnotKind.KFileV1,
            kind_id: r.id,
          },
        });
      }}
    />
  );
};

export default KFileBlock;
