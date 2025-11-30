"use client";

import React, { Suspense } from "react";

import { Button } from "@fe/components/button";

export const dynamic = "force-dynamic";

const Sidebar = async () => {
  const sidebarPromise = fetch(`http://localhost:5432/api/sidebar`).then(
    (res) => res.json(),
  );

  const [sidebar] = await Promise.all([sidebarPromise]);

  return (
    <>
      <div>sidebar: ${JSON.stringify(sidebar)}</div>
      <Button>testbutton</Button>
    </>
  );
};

const Statistics = async () => {
  const statisticsPromise = fetch(`http://localhost:5432/api/statistics`).then(
    (res) => res.json(),
  );
  const [statistics] = await Promise.all([statisticsPromise]);
  return (
    <>
      <div>statistics: ${JSON.stringify(statistics)}</div>
      <Button>testbutton2</Button>
    </>
  );
};

export default async function HomePage() {
  return (
    <>
      <Suspense fallback={<>Loading sidebar</>}>
        <Sidebar />
      </Suspense>
      <Suspense fallback={<>Loading statistics</>}>
        <Statistics />
      </Suspense>
    </>
  );
}
