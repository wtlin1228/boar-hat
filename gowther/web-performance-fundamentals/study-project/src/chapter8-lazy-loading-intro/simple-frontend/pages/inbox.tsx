import { useState } from "react";

import { FixedWidthPrimarySidebarSPA } from "@fe/patterns/fixed-width-primary-sidebar-spa";
import {
  Content,
  ContentBody,
  ContentHeading,
} from "@fe/patterns/layout-components";
import { TopbarForSidebarContentLayout } from "@fe/patterns/topbar-for-sidebar-content-layout";

import { MessageList } from "./patterns/messages-list";

export const InboxPage = () => {
  const [search, setSearch] = useState("");

  const onChange = (val: string) => {
    const cleanValue = val.trim().toLowerCase();

    // Send cleanValue to the server
    console.info(cleanValue);
  };
  return (
    <div className="blink-text-primary flex flex-col lg:flex-row h-screen bg-blinkGray50 dark:bg-blinkNeutral900 gap-0.5">
      <FixedWidthPrimarySidebarSPA />

      <div className="flex flex-1 h-full flex-col">
        <TopbarForSidebarContentLayout
          search={search}
          setSearch={(val) => {
            setSearch(val);
            onChange(val);
          }}
        />

        <div className="w-full h-full flex flex-col lg:flex-row">
          <Content>
            <ContentHeading>
              <h1 className="text-5xl blink-text-primary italic font-blink-title">
                Inbox
              </h1>
            </ContentHeading>
            <ContentBody>
              <MessageList />
            </ContentBody>
          </Content>
        </div>
      </div>
    </div>
  );
};
