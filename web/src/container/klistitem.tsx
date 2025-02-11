import clsx from "clsx";
import React, { HTMLProps } from "react";
import { ForwardedRef, ReactNode } from "react";

type KListItemProps = {
  children: ReactNode;
  focused?: boolean;
  className?: string;
} & Omit<React.LiHTMLAttributes<HTMLLIElement>, "className">;

const KListItem = React.forwardRef<HTMLLIElement, KListItemProps>(
  (props: KListItemProps, ref: ForwardedRef<HTMLLIElement>) => {
    const { children, focused, className, ...rest } = props;
    return (
      <li
        className={clsx(
          "flex p-3 pl-6 gap-1 relative select-none w-full group text-xs space-x-2",
          "hover:kbutton-focused",
          focused ? "kbutton-focused" : "kbutton border-transparent",
          className
        )}
        ref={ref}
        {...rest}
      >
        {children}
      </li>
    );
  }
);

export default KListItem;
