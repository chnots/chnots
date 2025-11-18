import Icon from "@/common/component/icon";
import { Button } from "@/common/component/ui/button";
import { SidebarTrigger } from "@/common/component/ui/sidebar";
import { ChnotKind } from "../../po";
import { ChnotKindIcon } from "../kind-icon";
import { cn } from "@/lib/utils";

const ChnotSingleHeadbar = ({
  kind,
  onNew,
  setKind,
  className,
}: {
  kind?: ChnotKind;
  onNew: () => void;
  setKind: (kind: ChnotKind) => void;
  className?: string;
}) => {
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
      {!kind && (
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
