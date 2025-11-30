import React from "react";

import { SidebarItemProps } from "@fe/components/sidebar/navigation-items";
import { merge } from "@fe/utils/merge-classnames";
import { usePath } from "@fe/utils/use-client-router";
import Link from "next/link";

export const SidebarLinkItem = React.forwardRef(
  (
    {
      before,
      after,
      children,
      href,
      onClick,
      isActive,
      className,
      ...rest
    }: SidebarItemProps,
    ref: React.ForwardedRef<never>,
  ) => {
    const Component = href ? Link : onClick ? "button" : "span";
    return (
      <Component
        ref={ref}
        className={merge(
          "flex items-center min-h-[3.375rem] sm:min-h-[2.125rem] rounded blink-double-focus-ring focus-visible:ring-offset-0",
          isActive ? "bg-blinkGray200 dark:bg-blinkGray700" : undefined,
          href || onClick
            ? "hover:bg-blinkGray100 dark:hover:bg-blinkGray800"
            : undefined,
          className,
        )}
        href={href as string}
        onClick={onClick}
        tabIndex={0}
        role={onClick ? "button" : undefined}
        {...rest}
      >
        {before && <span className="flex-shrink-0 inline-flex">{before}</span>}
        <span className="flex-grow inline-flex group-data-[sidebar-open=false]:hidden">
          {children}
        </span>
        {after && (
          <span className="flex-shrink-0 inline-flex group-data-[sidebar-open=false]:hidden">
            {after}
          </span>
        )}
      </Component>
    );
  },
);

export const SidebarRegularLinkItem = React.forwardRef(
  (
    { className, ...props }: SidebarItemProps,
    ref: React.ForwardedRef<never>,
  ) => {
    const path = usePath();
    return (
      <SidebarLinkItem
        ref={ref}
        className={merge("px-2 gap-3 sm:gap-2 cursor-pointer", className)}
        isActive={props.href === path}
        {...props}
      />
    );
  },
);
