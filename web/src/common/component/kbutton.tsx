import clsx from "clsx";
import { ButtonHTMLAttributes, ReactNode } from "react";

const KButton = ({
  children,
  showBorder,
  className,
  ...rest
}: {
  children: ReactNode;
  className?: string;
  showBorder?: boolean;
} & ButtonHTMLAttributes<object>) => {
  return (
    <div
      className={clsx(
        "flex flex-row items-center rounded-xl space-x-1 hover:cursor-pointer",
        showBorder ? "border-kbdr" : "border border-transparent",
        className
      )}
      {...rest}
    >
      {children}
    </div>
  );
};

export default KButton;
