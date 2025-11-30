import { AppLayout } from "@fe/patterns/app-layout";
import {
  Content,
  ContentBody,
  ContentHeading,
} from "@fe/patterns/layout-components";
import { MessageListFixed } from "@fe/patterns/messages-list-fixed";

export const InboxPage = () => {
  return (
    <AppLayout>
      <Content>
        <ContentHeading>
          <h1 className="text-5xl blink-text-primary italic font-blink-title">
            Inbox
          </h1>
        </ContentHeading>
        <ContentBody>
          <MessageListFixed />
        </ContentBody>
      </Content>
    </AppLayout>
  );
};
