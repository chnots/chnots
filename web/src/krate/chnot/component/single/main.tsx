import { useEffect, useState } from "react";
import { ChnotKind } from "../../po";
import { ChnotSingleBodyMemo } from "./body";
import ChnotSingleHeadbar from "./header";
import { genTID, TID } from "@/lib/id_util";
import { useChnotSingleStore } from "../../store";

export type ChnotSingleMainStore = {
  otid: TID;
  kind?: ChnotKind;
  setKind: (kind: ChnotKind) => void;
};

const ChnotSingleMain = () => {
  const { getMeta, setChangeCompCurOtid } = useChnotSingleStore((s) => {
    return {
      getMeta: s.getMeta,
      setChangeCompCurOtid: s.setChangeCompCurOtid,
    };
  });

  const [otid, setOtid] = useState<TID | undefined>(genTID());
  const [kind, setKind] = useState<ChnotKind | undefined>(undefined);

  useEffect(() => {
    setChangeCompCurOtid(setOtid);
  }, [setOtid]);

  useEffect(() => {
    if (otid) {
      setKind(getMeta(otid)?.meta.kind);
    } else {
      setKind(undefined);
    }
  }, [otid]);

  return (
    <div className="w-full h-full flex flex-col">
      <ChnotSingleHeadbar
        onNew={() => {
          setOtid(genTID());
        }}
        setKind={(kind: ChnotKind) => {
          setKind(kind);
        }}
        kind={kind}
      />
      {otid && (
        <ChnotSingleBodyMemo
          key={otid}
          otid={otid}
          kind={kind ?? ChnotKind.MDWT}
          setKind={setKind}
        />
      )}
    </div>
  );
};

export default ChnotSingleMain;
