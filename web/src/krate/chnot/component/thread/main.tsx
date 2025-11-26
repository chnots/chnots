import { useEffect, useState } from "react";
import { useKSpaceStore } from "@/krate/kspace/store";
import { genTID, type TID } from "@/lib/id_util";
import type { ChnotKind, ChnotThreadMeta } from "../../po";
import { useChnotThreadStore } from "../../store";
import ChnotThreadBody from "./body";
import ChnotThreadHeadbar from "./header";

export type ChnotSingleMainStore = {
  otid: TID;
  kind?: ChnotKind;
  setKind: (kind: ChnotKind) => void;
};

const ChnotThreadMain = () => {
  const { getMeta, setChangeCompCurOtid } = useChnotThreadStore((s) => {
    return {
      getMeta: s.getMeta,
      setChangeCompCurOtid: s.setChangeCompCurOtid,
    };
  });

  const [otid, setOtid] = useState<TID | undefined>(genTID());
  const [threadMeta, setThreadMeta] = useState<ChnotThreadMeta>();

  useEffect(() => {
    setChangeCompCurOtid(setOtid);
  }, [setChangeCompCurOtid]);

  const { kspace } = useKSpaceStore((s) => {
    return {
      kspace: s.currentKSpace,
    };
  });

  useEffect(() => {
    if (otid) {
      const meta = getMeta(otid);
      if (meta) {
        setThreadMeta(meta.meta);
      } else {
        setThreadMeta({
          otid: otid,
          kspace: kspace,
          tid: genTID(),
        });
      }
    }
  }, [otid, getMeta, kspace]);

  return (
    <div className="w-full h-full flex flex-col">
      <ChnotThreadHeadbar
        onNew={() => {
          setOtid(genTID());
        }}
      />
      {threadMeta && <ChnotThreadBody key={otid} threadMeta={threadMeta} />}
    </div>
  );
};

export default ChnotThreadMain;
