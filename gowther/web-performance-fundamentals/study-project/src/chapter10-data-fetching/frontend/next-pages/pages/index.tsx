import React from "react";

import { SidebarData } from "@fe/data/sidebar";
import { TableData } from "@fe/data/website-statistics";
import { DashboardWithDynamicDataWithPropsPage } from "@fe/pages/dashboard-with-props-data";

import { AppLayout } from "../components/app-layout";

export default function HomePage({
  statistics,
  sidebar,
}: {
  statistics: TableData[];
  sidebar: SidebarData;
}) {
  // props-drill the sidebar object down to the AppLayout and further to the sidebar itself
  return (
    <AppLayout>
      <DashboardWithDynamicDataWithPropsPage statistics={statistics} />
    </AppLayout>
  );
}

export const getServerSideProps = async () => {
  const sidebarPromise = fetch(`http://localhost:5432/api/sidebar`).then(
    (res) => res.json(),
  );
  const statisticsPromise = fetch(`http://localhost:5432/api/statistics`).then(
    (res) => res.json(),
  );

  const [sidebar, statistics] = await Promise.all([
    sidebarPromise,
    statisticsPromise,
  ]);

  // Pass data to the page via props
  return { props: { statistics, sidebar } };
};
