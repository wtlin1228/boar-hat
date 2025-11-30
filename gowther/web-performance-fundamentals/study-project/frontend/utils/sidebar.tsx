import { Badge } from "@fe/components/badge";
import { CollapsibleSubgroupRight } from "@fe/components/sidebar/navigation-groups";
import { SidebarRegularLinkItem } from "@fe/components/sidebar/navigation-items";
import { SidebarData } from "@fe/data/sidebar";
import { ClipboardIcon } from "@fe/icons/clipboard-icon";
import { DashboardIcon } from "@fe/icons/dashboard-icon";
import { FileIcon } from "@fe/icons/file-icon";
import { HomeIcon } from "@fe/icons/home-icon";
import { InboxIcon } from "@fe/icons/inbox-icon";
import { PercentageIcon } from "@fe/icons/percentage-icon";
import { SettingsIcon } from "@fe/icons/settings-icon";

export const renderIcon = (iconName: string) => {
  const iconProps = { className: "w-8 h-8 sm:w-6 sm:h-6" };

  switch (iconName) {
    case "home":
      return <HomeIcon {...iconProps} />;
    case "inbox":
      return <InboxIcon {...iconProps} />;
    case "percentage":
      return <PercentageIcon {...iconProps} />;
    case "dashboard":
      return <DashboardIcon {...iconProps} />;
    case "clipboard":
      return <ClipboardIcon {...iconProps} />;
    case "file":
      return <FileIcon {...iconProps} />;
    case "settings":
      return <SettingsIcon {...iconProps} />;
    default:
      return null;
  }
};

export const renderSidebar = (sidebarData?: SidebarData) => {
  if (!sidebarData) return undefined;

  return sidebarData.items.map((item, index) => {
    if (item.type === "link") {
      return (
        <SidebarRegularLinkItem
          key={index}
          href={item.href}
          before={renderIcon(item.icon)}
          after={item.badge ? <Badge text={item.badge} size="small" /> : null}
        >
          {item.text}
        </SidebarRegularLinkItem>
      );
    } else if (item.type === "collapsible") {
      return (
        <CollapsibleSubgroupRight
          key={index}
          text={item.text}
          icon={renderIcon(item.icon)}
        >
          {item.children.map((child, childIndex) => (
            <SidebarRegularLinkItem
              key={childIndex}
              href={child.href}
              className="pl-12"
              after={
                child.badge ? <Badge text={child.badge} size="small" /> : null
              }
            >
              {child.text}
            </SidebarRegularLinkItem>
          ))}
        </CollapsibleSubgroupRight>
      );
    }
    return null;
  });
};

export const SidebarSkeleton = () => {
  return (
    <>
      {[...Array(5)].map((_, i) => (
        <div key={i} className="flex items-center px-3 py-2 animate-pulse">
          <div className="w-5 h-5 rounded bg-blinkGray200 dark:bg-blinkNeutral700 mr-3"></div>
          <div className="h-4 bg-blinkGray200 dark:bg-blinkNeutral700 rounded w-20"></div>
          {i === 1 && (
            <div className="ml-auto w-5 h-4 rounded bg-blinkGray200 dark:bg-blinkNeutral700"></div>
          )}
        </div>
      ))}
      <div className="px-3 py-2 animate-pulse">
        <div className="flex items-center">
          <div className="w-5 h-5 rounded bg-blinkGray200 dark:bg-blinkNeutral700 mr-3"></div>
          <div className="h-4 bg-blinkGray200 dark:bg-blinkNeutral700 rounded w-16"></div>
        </div>
        <div className="ml-8 mt-2">
          {[...Array(3)].map((_, i) => (
            <div key={i} className="flex items-center py-1">
              <div className="h-4 bg-blinkGray200 dark:bg-blinkNeutral700 rounded w-16 mb-1"></div>
              {i !== 2 && (
                <div className="ml-auto w-5 h-4 rounded bg-blinkGray200 dark:bg-blinkNeutral700"></div>
              )}
            </div>
          ))}
        </div>
      </div>
      {[...Array(2)].map((_, i) => (
        <div key={i} className="flex items-center px-3 py-2 my-1 animate-pulse">
          <div className="w-5 h-5 rounded bg-blinkGray200 dark:bg-blinkNeutral700 mr-3"></div>
          <div className="h-4 bg-blinkGray200 dark:bg-blinkNeutral700 rounded w-24"></div>
        </div>
      ))}
    </>
  );
};
