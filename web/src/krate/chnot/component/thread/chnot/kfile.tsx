import { SaveState } from "@/common/types";
import { CommonKFile } from "@/krate/kfile/components/common-kfile";
import { ChnotChromeProps } from "./rich-chnot";

const KFileBlock = ({ otid, onPostSave }: ChnotChromeProps) => {
  return (
    <CommonKFile
      otid={otid}
      onPostSave={(r) => {
        onPostSave({
          saveState: SaveState.Saved,
        });
      }}
    />
  );
};

export default KFileBlock;
