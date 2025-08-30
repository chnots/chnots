import { RefObject } from "react";

import { SaveState } from "@/common/types";
import { ChnotMetaKind } from "../../vo";
import { TID } from "@/lib/id_util";
import { ChnotKind } from "@/krate/chnot/po";
import { CommonKFile } from "@/krate/kfile/components/common-kfile";

const KFileBlock = ({
  otid,
  kindId,
  setSaveState,
  blockKindsRef,
}: {
  otid: TID;
  kindId?: string;
  setSaveState: (saveState: SaveState) => void;
  blockKindsRef: RefObject<Map<TID, ChnotMetaKind>>;
}) => {
  return (
    <CommonKFile
      kindId={kindId}
      onPostSave={(r) => {
        setSaveState(SaveState.Saved);
        blockKindsRef.current.set(otid, {
          otid: otid,
          kind: ChnotKind.KFileV1,
          kind_id: r.id,
        });
      }}
    />
  );
};

export default KFileBlock;
