import * as React from "react";

import { createFileRoute } from "@tanstack/react-router";

import { LoginPage } from "../pages/login";

export const Route = createFileRoute("/login")({
  component: HomeComponent,
});

function HomeComponent() {
  return <LoginPage />;
}
