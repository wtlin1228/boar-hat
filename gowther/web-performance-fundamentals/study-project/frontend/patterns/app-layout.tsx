import { ReactNode, useState } from "react";

import { FixedWidthPrimarySidebarSPA } from "@fe/patterns/fixed-width-primary-sidebar-spa";
import { TopbarForSidebarContentLayout } from "@fe/patterns/topbar-for-sidebar-content-layout";

export const AppLayout = ({ children }: { children: ReactNode }) => {
  const [search, setSearch] = useState("");

  return (
    <div className="blink-text-primary flex flex-col lg:flex-row h-screen bg-blinkGray50 dark:bg-blinkNeutral900 gap-0.5">
      <FixedWidthPrimarySidebarSPA />

      <div className="flex flex-1 h-full flex-col">
        <TopbarForSidebarContentLayout search={search} setSearch={setSearch} />

        <div className="w-full h-full flex flex-col lg:flex-row">
          {children}
        </div>
      </div>
    </div>
  );
};
