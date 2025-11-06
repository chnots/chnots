import { SaveState } from "@/common/types";
import { CommonKFile } from "@/krate/kfile/components/common-kfile";
import { RichPropProps } from "./rich-chnot";

const KFileBlock = ({ otid, onPostSave }: RichPropProps) => {
  return (
    <CommonKFile
      otid={otid}
      onPostSave={(_) => {
        onPostSave({
          saveState: SaveState.Saved,
        });
      }}
    />
  );
};

export default KFileBlock;
