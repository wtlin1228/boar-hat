import * as React from "react";

import { createFileRoute } from "@tanstack/react-router";

import { SettingsPage } from "../pages/settings";

export const Route = createFileRoute("/settings")({
  component: HomeComponent,
});

function HomeComponent() {
  return <SettingsPage />;
}
