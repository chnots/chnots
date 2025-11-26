import clsx from "clsx";
import React, { type ForwardedRef, type ReactNode } from "react";

type KListItemProps = {
  children: ReactNode;
  focused?: boolean;
  className?: string;
} & Omit<React.LiHTMLAttributes<HTMLLIElement>, "className">;

const KListItem = React.forwardRef(
  (props: KListItemProps, ref: ForwardedRef<HTMLLIElement>) => {
    const { children, focused, className, ...rest } = props;
    return (
      <li
        className={clsx(
          "list-none p-3 relative select-none group text-xs hover:kc-active hover:cursor-pointer border rounded-xl",
          focused ? "kc-active" : "border-transparent",
          className,
        )}
        ref={ref}
        {...rest}
      >
        {children}
      </li>
    );
  },
);

KListItem.displayName = "KListItem";

export default KListItem;
