import { useCallback, useEffect, useState } from "react";
import { useKSpaceStore } from "@/krate/kspace/store";
import { genTID, type TID } from "@/lib/id_util";
import type { ChnotThreadMeta } from "../../po";
import { useChnotThreadStore } from "../../store";
import ChnotThreadBody from "./body";
import ChnotThreadHeadbar from "./header";

const ChnotThreadMain = () => {
  const { getMeta, setChangeCompCurOtid } = useChnotThreadStore((s) => {
    return {
      getMeta: s.getMeta,
      setChangeCompCurOtid: s.setChangeCompCurOtid,
    };
  });

  const [threadMeta, setThreadMeta] = useState<ChnotThreadMeta>();

  useEffect(() => {
    setChangeCompCurOtid(handleOtidChange);
  }, [setChangeCompCurOtid]);

  const { kspace } = useKSpaceStore((s) => {
    return {
      kspace: s.currentKSpace,
    };
  });

  const handleOtidChange = useCallback(
    (otid?: TID) => {
      if (otid) {
        const meta = getMeta(otid)?.meta ?? {
          otid: otid,
          kspace: kspace,
          tid: genTID(),
        };
        setThreadMeta(meta);
      }
    },
    [getMeta, kspace],
  );

  return (
    <div className="w-full h-full flex flex-col">
      <ChnotThreadHeadbar
        onNew={() => {
          handleOtidChange(genTID());
        }}
      />
      {threadMeta && (
        <ChnotThreadBody key={threadMeta.otid} threadMeta={threadMeta} />
      )}
    </div>
  );
};

export default ChnotThreadMain;
