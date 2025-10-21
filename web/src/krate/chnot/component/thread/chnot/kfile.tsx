import { SaveState } from "@/common/types";
import { ChnotKind } from "@/krate/chnot/po";
import { CommonKFile } from "@/krate/kfile/components/common-kfile";
import { ChnotChromeProps } from "./rich-chnot";

const KFileBlock = ({ kindId, onPostSave }: ChnotChromeProps) => {
  return (
    <CommonKFile
      kid={kindId}
      onPostSave={(r) => {
        onPostSave({
          saveState: SaveState.Saved,
          kindId: r.id,
        });
      }}
    />
  );
};

export default KFileBlock;
