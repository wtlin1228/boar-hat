import { useEffect, useState } from "react";

import { NavigationGroup } from "@fe/components/sidebar/navigation-groups";
import { PrimarySidebar } from "@fe/components/sidebar/sidebar-layouts";
import { SidebarData } from "@fe/data/sidebar";
import { PrimarySidebarBottomGroup } from "@fe/patterns/primary-sidebar-bottom-group";
import { PrimarySidebarCreateButton } from "@fe/patterns/primary-sidebar-create-button";
import { PrimarySidebarHeading } from "@fe/patterns/primary-sidebar-heading";
import { PrimarySidebarSecondaryGroup } from "@fe/patterns/primary-sidebar-secondary-group";
import { renderSidebar, SidebarSkeleton } from "@fe/utils/sidebar";

const PrimarySidebarPrimaryGroupSPA = ({ ...props }) => {
  const [sidebarData, setSidebarData] = useState<SidebarData | undefined>();
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const fetchSidebarData = async () => {
      try {
        // replace with your port if you changed it
        const response = await fetch("http://localhost:5432/api/sidebar");
        if (!response.ok) {
          throw new Error("Failed to fetch sidebar data");
        }
        const data = await response.json();
        setSidebarData(data);
      } catch (_) {
        // deal with the errors here
      } finally {
        setIsLoading(false);
      }
    };

    fetchSidebarData();
  }, []);

  return (
    <NavigationGroup header="general" {...props}>
      {isLoading ? <SidebarSkeleton /> : renderSidebar(sidebarData)}
    </NavigationGroup>
  );
};

export const FixedWidthPrimarySidebarSPA = () => {
  return (
    <PrimarySidebar>
      <PrimarySidebarHeading />

      <PrimarySidebarCreateButton />

      <PrimarySidebarPrimaryGroupSPA />

      <PrimarySidebarSecondaryGroup />

      <div className="flex-grow" />

      <PrimarySidebarBottomGroup />
    </PrimarySidebar>
  );
};
