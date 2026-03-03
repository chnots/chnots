import { Circle, CircleQuestionMark, Square, Triangle, X } from "lucide-react";
import { SaveState } from "@/common/types";

const BlockState = ({ saveState }: { saveState: SaveState }) => {
  const getStateInfo = () => {
    switch (saveState) {
      case SaveState.Dirty:
        return {
          icon: <Square className="w-4 h-4 text-gray-400 m-1" />,
          text: "Modified",
          color: "text-green-500",
        };
      case SaveState.Saved:
        return {
          icon: <Circle className="w-4 h-4 text-gray-400 m-1" />,
          text: "Saved",
          color: "text-green-500",
        };
      case SaveState.Saving:
        return {
          icon: <Triangle className="w-4 h-4 text-gray-400 m-1 animate-spin" />,
          text: "Saving...",
          color: "text-green-500",
        };
      case SaveState.Error:
        return {
          icon: <X className="w-4 h-4 text-gray-400 m-1" />,
          text: "Failed",
          color: "text-red-500",
        };
      default:
        return {
          icon: <CircleQuestionMark className="w-4 h-4 text-gray-400 m-1" />,
          text: "Unknown",
        };
    }
  };

  const stateInfo = getStateInfo();
  return stateInfo.icon;
};

export default BlockState;
