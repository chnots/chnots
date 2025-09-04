import { SaveState } from "@/common/types";
import { ChnotKind } from "@/krate/chnot/po";
import { CommonKFile } from "@/krate/kfile/components/common-kfile";
import { ChnotChromeProps } from "./chrome";

const KFileBlock = ({ chnotOtid, kindId, onPostSave }: ChnotChromeProps) => {
  return (
    <CommonKFile
      kid={kindId}
      onPostSave={(r) => {
        onPostSave({
          saveState: SaveState.Saved,
          data: {
            chnotOtid,
            kind: ChnotKind.KFileV1,
            kindId: r.id,
          },
        });
      }}
    />
  );
};

export default KFileBlock;
