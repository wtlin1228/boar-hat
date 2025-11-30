import React, { lazy, ReactNode, Suspense, useState } from "react";

import { Drawer } from "@fe/components/drawer";
import { TopbarForSidebarContentLayout } from "@fe/patterns/topbar-for-sidebar-content-layout";
import { merge } from "@fe/utils/merge-classnames";
import * as DialogPrimitives from "@radix-ui/react-dialog";

const FixedWidthPrimarySidebarSPALazy = lazy(async () => {
  const spa = await import("@fe/patterns/fixed-width-primary-sidebar-spa");

  return {
    default: spa.FixedWidthPrimarySidebarSPA,
  };
});

export const AppLayoutLazySidebar = ({
  children,
  search,
  setSearch,
}: {
  children: ReactNode;
  search: string;
  setSearch: (v: string) => void;
}) => {
  return (
    <div className="blink-text-primary flex flex-col lg:flex-row h-screen bg-blinkGray50 dark:bg-blinkNeutral900 gap-0.5">
      <Suspense fallback={<div className="sidebar-fallback"></div>}>
        <FixedWidthPrimarySidebarSPALazy />
      </Suspense>

      <div className="flex flex-1 h-full flex-col">
        <TopbarForSidebarContentLayout search={search} setSearch={setSearch} />

        <div className="w-full h-full flex flex-col lg:flex-row">
          {children}
        </div>
      </div>
    </div>
  );
};
