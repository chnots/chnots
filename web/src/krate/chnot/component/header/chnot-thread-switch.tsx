import { NavLink } from "react-router-dom";
import Icon from "@/common/component/icon";
import { RoutePaths } from "@/router";
import { ChnotViewType } from "../../store";

const ChnotThreadSwitch = ({ viewType }: { viewType: ChnotViewType }) => {
  return viewType === ChnotViewType.Single ? (
    <NavLink to={RoutePaths.ChnotThread} id={"chnot"}>
      <Icon.LineSquiggle className="w-4 h-4" />
    </NavLink>
  ) : (
    <NavLink to={RoutePaths.Chnots} id={"chnot"}>
      <Icon.Spool className="w-4 h-4" />
    </NavLink>
  );
};

export default ChnotThreadSwitch;
