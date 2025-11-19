import Icon from '@/common/component/icon';
import { SaveState } from '@/common/types';

const BlockState = ({ saveState }: { saveState: SaveState }) => {
  const getStateInfo = () => {
    switch (saveState) {
      case SaveState.Dirty:
        return {
          icon: <Icon.Square className="w-4 h-4 text-gray-400 m-1" />,
          text: 'Modified',
          color: 'text-green-500',
        };
      case SaveState.Saved:
        return {
          icon: <Icon.Circle className="w-4 h-4 text-gray-400 m-1" />,
          text: 'Saved',
          color: 'text-green-500',
        };
      case SaveState.Saving:
        return {
          icon: <Icon.Triangle className="w-4 h-4 text-gray-400 m-1 animate-spin" />,
          text: 'Saving...',
          color: 'text-green-500',
        };
      case SaveState.Error:
        return {
          icon: <Icon.X className="w-4 h-4 text-gray-400 m-1" />,
          text: 'Failed',
          color: 'text-red-500',
        };
      default:
        return {
          icon: <Icon.CircleQuestionMark className="w-4 h-4 text-gray-400 m-1" />,
          text: 'Unknown',
        };
    }
  };

  const stateInfo = getStateInfo();
  return stateInfo.icon;
};

export default BlockState;
