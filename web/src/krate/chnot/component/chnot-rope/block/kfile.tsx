import { RefObject } from "react";

import { SaveState } from "@/common/types";
import { ChnotBlockMetaKind } from "../../vo";
import { TID } from "@/lib/id_util";
import { ChnotKind } from "@/krate/chnot/po";
import { CommonKFile } from "@/krate/kfile/components/common-kfile";

const KFileBlock = ({
  otid,
  kindId,
  readonly,
  setSaveState,
  blockKindsRef,
}: {
  otid: TID;
  kindId?: string;
  readonly?: boolean;
  setSaveState: (saveState: SaveState) => void;
  blockKindsRef: RefObject<Map<TID, ChnotBlockMetaKind>>;
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
