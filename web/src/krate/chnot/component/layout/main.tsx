import { useEffect, useState } from "react";
import { genTID, type TID } from "@/lib/id_util";
import { cn } from "@/lib/utils";
import { ChnotKind } from "../../po";
import { useChnotStore } from "../../store";
import { ChnotBodyMemo } from "./body";
import ChnotHeadbar from "./header";

const ChnotSingleMain = ({ className }: { className?: string }) => {
  const { getMeta, setChangeCompCurOtid } = useChnotStore((s) => {
    return {
      getMeta: s.getMeta,
      setChangeCompCurOtid: s.setChangeCompCurOtid,
    };
  });

  const [otid, setOtid] = useState<TID | undefined>(genTID());
  const [kind, setKind] = useState<ChnotKind | undefined>(undefined);

  useEffect(() => {
    setChangeCompCurOtid(setOtid);
  }, [setChangeCompCurOtid]);

  useEffect(() => {
    if (otid) {
      setKind(getMeta(otid)?.meta.kind);
    } else {
      setKind(undefined);
    }
  }, [otid, getMeta]);

  return (
    <main className={cn("relative flex flex-col overflow-y-auto", className)}>
      <ChnotHeadbar
        className={"sticky top-0 left-0"}
        onNew={() => {
          setOtid(genTID());
        }}
        setKind={(kind: ChnotKind) => {
          setKind(kind);
        }}
        otid={otid}
      />
      {otid && (
        <ChnotBodyMemo key={otid} otid={otid} kind={kind ?? ChnotKind.MDWT} />
      )}
    </main>
  );
};

export default ChnotSingleMain;
