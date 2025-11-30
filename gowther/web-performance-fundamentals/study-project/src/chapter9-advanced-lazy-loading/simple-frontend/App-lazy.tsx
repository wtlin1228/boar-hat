import { lazy, Suspense } from "react";

import { usePath } from "@fe/utils/use-client-router";

const SettinsPageLazy = lazy(async () => {
  return {
    default: (await import("./pages/settings")).SettingsPage,
  };
});
const LoginPageLazy = lazy(async () => {
  return {
    default: (await import("./pages/login")).LoginPage,
  };
});
const InboxPageLazy = lazy(async () => {
  return {
    default: (await import("./pages/inbox")).InboxPage,
  };
});

const DashboardPageLazy = lazy(async () => {
  return {
    default: (await import("./pages/dashboard")).DashboardPage,
  };
});

export default function App({ ssrPath }: { ssrPath?: string }) {
  const path = usePath(ssrPath);

  if (path.startsWith("/settings")) {
    return (
      <Suspense>
        <SettinsPageLazy />
      </Suspense>
    );
  }

  switch (path) {
    case "/login":
      return (
        <Suspense>
          <LoginPageLazy />
        </Suspense>
      );

    case "/inbox":
      return (
        <Suspense>
          <InboxPageLazy />
        </Suspense>
      );
    default:
      return (
        <Suspense>
          <DashboardPageLazy />
        </Suspense>
      );
  }
}
