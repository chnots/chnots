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
    <button
      className={clsx(
        "flex flex-row items-center hover:kbutton-focused rounded-xl space-x-1",
        showBorder ? "kbutton bg-secondary" : "border border-transparent",
        className
      )}
      {...rest}
    >
      {children}
    </button>
  );
};

export default KButton;
