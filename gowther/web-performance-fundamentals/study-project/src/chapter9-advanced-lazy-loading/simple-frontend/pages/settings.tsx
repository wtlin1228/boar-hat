import { TabButton, TabContent, TabsList, TabsRoot } from "@fe/components/tabs";
import { AppLayout } from "@fe/patterns/app-layout";
import {
  Content,
  ContentBody,
  ContentHeading,
} from "@fe/patterns/layout-components";
import { SettingsNotificationsTable } from "@fe/patterns/settings-notifications-table";
import { SettingsProfileForm } from "@fe/patterns/settings-profile-form";

export const SettingsPage = () => {
  return (
    <AppLayout>
      <Content>
        <ContentHeading>
          <h1 className="text-5xl blink-text-primary italic font-blink-title">
            Settings
          </h1>
        </ContentHeading>
        <ContentBody>
          <TabsRoot defaultValue="notifications">
            <TabsList className="mb-6">
              <TabButton value="account-details">Account details</TabButton>
              <TabButton value="personal-profile">Personal Profile</TabButton>
              <TabButton value="email-settings">Email Settings</TabButton>
              <TabButton value="notifications">Notifications</TabButton>
            </TabsList>
            <div className="py-4">
              <TabContent value="account-details">
                Content for Account details
              </TabContent>
              <TabContent value="personal-profile">
                <SettingsProfileForm />
              </TabContent>
              <TabContent value="email-settings">
                Content for Email Settings
              </TabContent>
              <TabContent value="notifications">
                <SettingsNotificationsTable />
              </TabContent>
            </div>
          </TabsRoot>
        </ContentBody>
      </Content>
    </AppLayout>
  );
};
