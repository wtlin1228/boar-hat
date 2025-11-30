import { Suspense } from "react";

import { NavigationGroup } from "@fe/components/sidebar/navigation-groups";
import { PrimarySidebar } from "@fe/components/sidebar/sidebar-layouts";
import { SidebarData } from "@fe/data/sidebar";
import { PrimarySidebarBottomGroup } from "@fe/patterns/primary-sidebar-bottom-group";
import { PrimarySidebarCreateButton } from "@fe/patterns/primary-sidebar-create-button";
import { PrimarySidebarSecondaryGroup } from "@fe/patterns/primary-sidebar-secondary-group";
import { renderSidebar, SidebarSkeleton } from "@fe/utils/sidebar";

import { PrimarySidebarHeading } from "./primary-sidebar-heading";

const PrimarySidebarPrimaryGroupSPA = async ({ ...props }) => {
  const sidebarResponse = await fetch("http://localhost:5432/api/sidebar");
  const sidebarData = (await sidebarResponse.json()) as SidebarData;

  return (
    <NavigationGroup header="general" {...props}>
      {renderSidebar(sidebarData)}
    </NavigationGroup>
  );
};

export const FixedWidthPrimarySidebar = () => {
  return (
    <PrimarySidebar>
      <PrimarySidebarHeading />

      <PrimarySidebarCreateButton />

      <Suspense fallback={<SidebarSkeleton />}>
        <PrimarySidebarPrimaryGroupSPA />
      </Suspense>

      <PrimarySidebarSecondaryGroup />

      <div className="flex-grow" />

      <PrimarySidebarBottomGroup />
    </PrimarySidebar>
  );
};
