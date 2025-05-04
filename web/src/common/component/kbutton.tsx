import clsx from "clsx";
import { ButtonHTMLAttributes, ReactNode } from "react";

const KButton = ({
  children,
  onClick,
  showBorder,
  className,
  ...rest
}: {
  children: ReactNode;
  className?: string;
  showBorder?: boolean;
  onClick?: () => void;
} & ButtonHTMLAttributes<object>) => {
  return (
    <button
      className={clsx(
        "flex flex-row items-center hover:kbutton-focused rounded-xl",
        showBorder ? "kbutton" : "border border-transparent",
        className
      )}
      onClick={onClick}
      {...rest}
    >
      {children}
    </button>
  );
};

export default KButton;
