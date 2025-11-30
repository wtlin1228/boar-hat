import { lazy, Suspense, useState } from "react";

import { usePath } from "@fe/utils/use-client-router";

import { AppLayoutLazySidebar } from "./components/app-layout";

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
  const [search, setSearch] = useState("");

  const searchResults =
    "hidden fixed p-10 top-12 right-10 w-1/3 bg-blinkBlue100";

  if (path.startsWith("/settings")) {
    return (
      <AppLayoutLazySidebar search={search} setSearch={setSearch}>
        <Suspense>
          <SettinsPageLazy />
        </Suspense>
        <div className={searchResults}>Search results for {search}</div>
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
        <AppLayoutLazySidebar search={search} setSearch={setSearch}>
          <Suspense>
            <InboxPageLazy />
          </Suspense>
          <div className={searchResults}>Search results for {search}</div>
        </AppLayoutLazySidebar>
      );
    default:
      return (
        <AppLayoutLazySidebar search={search} setSearch={setSearch}>
          <Suspense>
            <DashboardPageLazy />
          </Suspense>
          <div className={searchResults}>Search results for {search}</div>
        </AppLayoutLazySidebar>
      );
  }
}
