import { usePath } from "@fe/utils/use-client-router";

import { DashboardPage } from "./pages/dashboard";
import { LoginPage } from "./pages/login";
import { SettingsPage } from "./pages/settings";

function App() {
  const path = usePath();

  if (path.startsWith("/settings")) {
    return <SettingsPage />;
  }

  switch (path) {
    case "/login":
      return <LoginPage />;
    default:
      return <DashboardPage />;
  }
}

export default App;
