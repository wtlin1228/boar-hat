import * as React from "react";

import { createFileRoute } from "@tanstack/react-router";

import { InboxPage } from "../pages/inbox";

export const Route = createFileRoute("/inbox")({
  component: HomeComponent,
});

function HomeComponent() {
  return <InboxPage />;
}
