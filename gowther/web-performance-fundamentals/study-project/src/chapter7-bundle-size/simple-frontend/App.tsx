import { usePath } from "@fe/utils/use-client-router";

import { DashboardPage } from "./pages/dashboard";
import { InboxPage } from "./pages/inbox";
import { LoginPage } from "./pages/login";
import { SettingsPage } from "./pages/settings";

export default function App({ ssrPath }: { ssrPath?: string }) {
  const path = usePath(ssrPath);

  if (path.startsWith("/settings")) {
    return <SettingsPage />;
  }

  switch (path) {
    case "/login":
      return <LoginPage />;
    case "/inbox":
      return <InboxPage />;
    default:
      return <DashboardPage />;
  }
}
