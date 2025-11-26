import { SaveState } from "@/common/types";
import { CommonKFile } from "@/krate/kfile/components/common-kfile";
import type { RichPropProps } from "./rich-chnot";

const KFileChnot = ({ otid, onPostSave }: RichPropProps) => {
  return (
    <div>
      <CommonKFile
        otid={otid}
        onPostSave={(f) => {
          onPostSave({
            otid,
            saveState: SaveState.Saved,
            title: f.filename,
          });
        }}
      />
    </div>
  );
};

export default KFileChnot;
