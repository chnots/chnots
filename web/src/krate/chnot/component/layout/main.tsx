import { useEffect, useState } from "react";
import { genTID, type TID } from "@/lib/id_util";
import { cn } from "@/lib/utils";
import { ChnotKind } from "../../po";
import { chnotMetaList } from "../../service";
import { useChnotStore } from "../../store";
import { ChnotBodyMemo } from "./body";
import ChnotHeadbar from "./header";

const ChnotMain = ({
  className,
  initialOtid,
  hideSidebar = false,
}: {
  className?: string;
  initialOtid?: TID;
  hideSidebar?: boolean;
}) => {
  const { getMeta, setChangeCompCurOtid, setCurOtid } = useChnotStore((s) => {
    return {
      getMeta: s.getMeta,
      setChangeCompCurOtid: s.setChangeCompCurOtid,
      setCurOtid: s.setCurOtid,
    };
  });

  const [otid, setOtid] = useState<TID | undefined>(
    () => initialOtid ?? genTID(),
  );
  const [kind, setKind] = useState<ChnotKind | undefined>(undefined);

  useEffect(() => {
    setChangeCompCurOtid(setOtid);
  }, [setChangeCompCurOtid]);

  useEffect(() => {
    let cancelled = false;
    if (otid) {
      const cached = getMeta(otid)?.meta.kind;
      if (cached) {
        setKind(cached);
      } else {
        setKind(undefined);
        void chnotMetaList({ otids: [otid] }).then((rsp) => {
          if (!cancelled) {
            setKind(rsp.metas[0]?.kind);
          }
        });
      }
    } else {
      setKind(undefined);
    }
    setCurOtid(otid);
    return () => {
      cancelled = true;
    };
  }, [getMeta, otid, setCurOtid]);

  return (
    <main
      className={cn(
        "relative flex min-h-0 flex-col overflow-hidden bg-background",
        !hideSidebar && "border-l",
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
        hideSidebar={hideSidebar}
      />
      <div className="min-h-0 flex-1 overflow-hidden">
        {otid && (
          <ChnotBodyMemo key={otid} otid={otid} kind={kind ?? ChnotKind.MDWT} />
        )}
      </div>
    </main>
  );
};

export default ChnotMain;
