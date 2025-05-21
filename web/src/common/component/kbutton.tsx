import clsx from "clsx";
import { ButtonHTMLAttributes, ReactNode } from "react";

export type KButtonProps = {
  children: ReactNode;
  className?: string;
  showBorder?: boolean;
  falseButton?: boolean;
} & ButtonHTMLAttributes<object>;

const KButton = ({
  children,
  showBorder,
  className,
  falseButton,
  ...rest
}: KButtonProps) => {
  return falseButton ? (
    <div
      className={clsx(
        "flex flex-row items-center rounded-xl space-x-1",
        showBorder ? "border-kbdr" : "border border-transparent",
        rest.disabled ? "hover:cursor-not-allowed" : "hover:cursor-pointer",
        className
      )}
      {...rest}
    >
      {children}
    </div>
  ) : (
    <button
      className={clsx(
        "flex flex-row items-center rounded-xl space-x-1 ",
        showBorder ? "border-kbdr" : "border border-transparent",
        rest.disabled ? "hover:cursor-not-allowed" : "hover:cursor-pointer",
        className
      )}
      {...rest}
    >
      {children}
    </button>
  );
};

export default KButton;
