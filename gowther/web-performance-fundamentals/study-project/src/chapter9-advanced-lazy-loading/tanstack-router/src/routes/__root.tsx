import * as React from "react";

import { AppLayout } from "@fe/patterns/app-layout-tr";
import { Outlet, createRootRoute, useLocation } from "@tanstack/react-router";

export const Route = createRootRoute({
  component: RootComponent,
});

function RootComponent() {
  // Get current pathname
  const { pathname } = useLocation();

  // If it's the login route, don't use AppLayout
  if (pathname === "/login") {
    return <Outlet />;
  }

  // Otherwise, use AppLayout for all other routes
  return (
    <AppLayout>
      <Outlet />
    </AppLayout>
  );
}
