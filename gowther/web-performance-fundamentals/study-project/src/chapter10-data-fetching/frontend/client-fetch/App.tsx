import { lazy, Suspense } from "react";

import { AppLayoutLazySidebarDynamic } from "@fe/patterns/app-layout-lazy-sidebar-dynamic";
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
      <AppLayoutLazySidebarDynamic>
        <Suspense>
          <SettinsPageLazy />
        </Suspense>
      </AppLayoutLazySidebarDynamic>
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
        <AppLayoutLazySidebarDynamic>
          <Suspense>
            <InboxPageLazy />
          </Suspense>
        </AppLayoutLazySidebarDynamic>
      );
    default:
      return (
        <AppLayoutLazySidebarDynamic>
          <Suspense>
            <DashboardPageLazy />
          </Suspense>
        </AppLayoutLazySidebarDynamic>
      );
  }
}
