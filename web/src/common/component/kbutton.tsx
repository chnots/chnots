import clsx from "clsx";
import { ReactNode } from "react";

const KButton = ({
  children,
  onClick,
  className,
}: {
  className?: string;
  children: ReactNode;
  onClick?: () => void;
}) => {
  return (
    <div
      className={clsx(
        "flex flex-row items-center kbutton hover:kbutton-focused rounded-xl space-x-2",
        className
      )}
      onClick={onClick}
    >
      {children}
    </div>
  );
};

export default KButton;
