import Icon from "@/common/component/icon";
import { SaveState } from "@/common/types";

const BlockState = ({ saveState }: { saveState: SaveState }) => {
  const getStateInfo = () => {
    switch (saveState) {
      case SaveState.Dirty:
        return {
          icon: <Icon.Square className="w-4 h-4" />,
          text: "Modified",
          color: "text-green-500",
        };
      case SaveState.Saved:
        return {
          icon: <Icon.Circle className="w-4 h-4" />,
          text: "Saved",
          color: "text-green-500",
        };
      case SaveState.Saving:
        return {
          icon: <Icon.Triangle className="w-4 h-4 animate-spin" />,
          text: "Saving...",
          color: "text-green-500",
        };
      case SaveState.Error:
        return {
          icon: <Icon.X className="w-4 h-4" />,
          text: "Failed",
          color: "text-red-500",
        };
      default:
        return {
          icon: <Icon.CircleQuestionMark />,
          text: "Unknown",
          color: "text-gray-500",
        };
    }
  };

  const stateInfo = getStateInfo();
  return (
    <div
      className={`flex items-center gap-2 border rounded p-1`}
      role="status"
      aria-live="polite"
      aria-label={`Save State: ${stateInfo.text}`}
    >
      <span className={stateInfo.color}>{stateInfo.icon}</span>
    </div>
  );
};

export default BlockState;
