import clsx from "clsx";
import React from "react";
import { ForwardedRef, ReactNode } from "react";

type KListItemProps = {
  children: ReactNode;
  focused?: boolean;
  className?: string;
} & Omit<React.LiHTMLAttributes<HTMLLIElement>, "className">;

const KListItem = React.forwardRef(
  (props: KListItemProps, ref: ForwardedRef<HTMLLIElement>) => {
    const { children, focused, className, ...rest } = props;
    return (
      <div>
        <li
          className={clsx(
            "list-none rounded-sm p-3 grid gap-1 relative select-none group",
            "hover:kbutton-focused",
            focused ? "kbutton-focused" : "kbutton border-transparent",
            className
          )}
          ref={ref}
          {...rest}
        >
          {children}
        </li>
      </div>
    );
  }
);

KListItem.displayName = "KListItem";

export default KListItem;
