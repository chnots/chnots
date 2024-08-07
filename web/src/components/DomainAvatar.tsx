import clsx from "clsx";
import Icon from "./Icon";

interface Props {
  avatarUrl?: string;
  className?: string;
}

const DomainAvatar = (props: Props) => {
  const { className } = props;
  return (
    <div
      className={clsx(
        `w-8 h-8 overflow-clip rounded-xl items-center justify-center`,
        className
      )}
    >
      <Icon.Earth />
    </div>
  );
};

export default DomainAvatar;
