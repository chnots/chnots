import { SaveState } from "@/common/types";
import { KFileViewer } from "@/krate/kfile/components";
import { ChnotKind } from "../../po";
import type { RichPropProps } from "./rich-mdwt-side";
import Fullscreen from "./fullscreen";

const KFileChnot = ({
  otid,
  fullscreen,
  readonly,
  onPostSave,
  onSetFullscreen,
}: RichPropProps) => {
  return (
    <div>
      <KFileViewer
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
      {fullscreen && (
        <Fullscreen onSetFullscreen={onSetFullscreen}>
          <KFileViewer
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
        </Fullscreen>
      )}
    </div>
  );
};

export default KFileChnot;
