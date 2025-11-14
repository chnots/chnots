import Icon from "@/common/component/icon";
import { Button } from "@/common/component/ui/button";
import { SidebarTrigger } from "@/common/component/ui/sidebar";
import { ChnotKind } from "../../po";
import { ChnotKindIcon } from "../kind-icon";

const ChnotThreadHeadbar = ({ onNew }: { onNew: () => void }) => {
  return (
    <div className="w-full flex align-center items-center p-1 space-x-1">
      <SidebarTrigger />
      <div className="m-1">
        <Button onClick={onNew}>
          <Icon.BadgePlusIcon />
        </Button>
      </div>
    </div>
  );
};

export default ChnotThreadHeadbar;
