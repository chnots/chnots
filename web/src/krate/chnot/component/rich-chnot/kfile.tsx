import { SaveState } from "@/common/types";
import { KFileViewer } from "@/krate/kfile/components";
import { ChnotKind } from "../../po";
import Fullscreen from "./fullscreen";
import type { RichPropProps } from "./rich-mdwt-side";

const KFileChnot = ({
  otid,
  fullscreen,
  readonly,
  onPostSave,
  onSetFullscreen,
  disableHeaderActions,
}: RichPropProps) => {
  return (
    <div className="w-full h-full">
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
        readonly={readonly}
        disableHeaderActions={disableHeaderActions}
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
            disableHeaderActions={disableHeaderActions}
          />
        </Fullscreen>
      )}
    </div>
  );
};

export default KFileChnot;
