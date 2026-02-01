import { SaveState } from "@/common/types";
import { CommonKFile } from "@/krate/kfile/components/common-kfile";
import type { RichPropProps } from "./rich-mdwt-side";
import { ChnotKind } from "../../po";

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
            kind: ChnotKind.KFileV1,
          });
        }}
      />
    </div>
  );
};

export default KFileChnot;
