import * as React from "react";

import { createFileRoute } from "@tanstack/react-router";

import { DashboardPage } from "../pages/dashboard";

export const Route = createFileRoute("/")({
  component: HomeComponent,
});

function HomeComponent() {
  return <DashboardPage />;
}
