import React, { Suspense } from "react";

import { AppLayout } from "../../components/app-layout";
import { DashboardRSC } from "../../components/dashboard";

export const dynamic = "force-dynamic";

export default function HomePage() {
  return (
    <AppLayout>
      <Suspense fallback={<>Loading dashboard</>}>
        <DashboardRSC />
      </Suspense>
    </AppLayout>
  );
}
