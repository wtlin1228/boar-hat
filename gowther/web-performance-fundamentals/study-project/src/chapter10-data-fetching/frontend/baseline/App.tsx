import { lazy, Suspense } from "react";

import { AppLayoutLazySidebar } from "@fe/patterns/app-layout-lazy-sidebar";
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
      <AppLayoutLazySidebar>
        <Suspense>
          <SettinsPageLazy />
        </Suspense>
      </AppLayoutLazySidebar>
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
        <AppLayoutLazySidebar>
          <Suspense>
            <InboxPageLazy />
          </Suspense>
        </AppLayoutLazySidebar>
      );
    default:
      return (
        <AppLayoutLazySidebar>
          <Suspense>
            <DashboardPageLazy />
          </Suspense>
        </AppLayoutLazySidebar>
      );
  }
}
