import clsx from "clsx";
import { ReactNode } from "react";

const KButton = ({
  children,
  onClick,
  showBorder,
  className,
}: {
  children: ReactNode;
  className?: string;
  showBorder?: boolean;
  onClick?: () => void;
}) => {
  return (
    <div
      className={clsx(
        "flex flex-row items-center hover:kbutton-focused rounded-xl space-x-2 p-2",
        showBorder ? "kbutton" : "border border-transparent",
        className
      )}
      onClick={onClick}
    >
      {children}
    </div>
  );
};

export default KButton;
