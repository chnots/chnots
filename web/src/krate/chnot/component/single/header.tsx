import Icon from "@/common/component/icon";
import { Button } from "@/common/component/ui/button";
import { SidebarTrigger } from "@/common/component/ui/sidebar";
import type { TID } from "@/lib/id_util";
import { cn } from "@/lib/utils";
import { ChnotKind } from "../../po";
import { useChnotSingleStore } from "../../store";
import { ChnotKindIcon } from "../kind-icon";

const ChnotSingleHeadbar = ({
  otid,
  onNew,
  setKind,
  className,
}: {
  otid?: TID;
  onNew: () => void;
  setKind: (kind: ChnotKind) => void;
  className?: string;
}) => {
  const { mapByOtid: _, getMeta } = useChnotSingleStore((s) => {
    return {
      mapByOtid: s.mapByOtid,
      getMeta: s.getMeta,
    };
  });

  return (
    <div
      className={cn(
        "w-full flex align-center items-center p-1 space-x-1",
        className,
      )}
    >
      <SidebarTrigger />
      <div className="m-1">
        <Button onClick={onNew}>
          <Icon.BadgePlusIcon />
        </Button>
      </div>
      {(!otid || !getMeta(otid)) && (
        <div className="rounded-md border p-0 m-0">
          {Object.values(ChnotKind).map((kind) => {
            return (
              <Button
                key={kind}
                onClick={() => {
                  setKind(kind);
                }}
                variant={"ghost"}
              >
                <ChnotKindIcon kind={kind} />
              </Button>
            );
          })}
        </div>
      )}
    </div>
  );
};

export default ChnotSingleHeadbar;
