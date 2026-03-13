import { useEffect, useState } from "react";
import { genTID, type TID } from "@/lib/id_util";
import { cn } from "@/lib/utils";
import { ChnotKind } from "../../po";
import { useChnotStore } from "../../store";
import { ChnotBodyMemo } from "./body";
import ChnotHeadbar from "./header";

const ChnotMain = ({ className }: { className?: string }) => {
  const { getMeta, setChangeCompCurOtid, setCurOtid } = useChnotStore((s) => {
    return {
      getMeta: s.getMeta,
      setChangeCompCurOtid: s.setChangeCompCurOtid,
      setCurOtid: s.setCurOtid,
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
    setCurOtid(otid);
  }, [otid]);

  return (
    <main
      className={cn(
        "relative flex flex-col min-h-0 overflow-hidden",
        className,
      )}
    >
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
      <div className="flex-1 min-h-0 overflow-hidden">
        {otid && (
          <ChnotBodyMemo key={otid} otid={otid} kind={kind ?? ChnotKind.MDWT} />
        )}
      </div>
    </main>
  );
};

export default ChnotMain;
