export type SidebarLinkItem = {
  type: "link";
  href: string;
  text: string;
  icon: string;
  badge: string | undefined;
};

export type SidebarCollapsibleItem = {
  type: "collapsible";
  text: string;
  icon: string;
  children: {
    type: "link";
    href: string;
    text: string;
    badge: string;
  }[];
};

export type SidebarItem = SidebarLinkItem | SidebarCollapsibleItem;

export type SidebarData = {
  header: string;
  items: SidebarItem[];
};

export const sidebarData: SidebarData = {
  header: "general",
  items: [
    {
      type: "link",
      href: "/",
      text: "Home",
      icon: "home",
      badge: undefined,
    },
    {
      type: "link",
      href: "/inbox",
      text: "Inbox",
      icon: "inbox",
      badge: "24",
    },
    {
      type: "link",
      href: "#",
      text: "Reporting",
      icon: "percentage",
      badge: undefined,
    },
    {
      type: "link",
      href: "#",
      text: "Dashboard",
      icon: "dashboard",
      badge: undefined,
    },
    {
      type: "collapsible",
      text: "Tasks",
      icon: "clipboard",
      children: [
        {
          type: "link",
          href: "#",
          text: "Todo",
          badge: "1",
        },
        {
          type: "link",
          href: "#",
          text: "In progress",
          badge: "11",
        },
        {
          type: "link",
          href: "#",
          text: "Done",
          badge: "56",
        },
      ],
    },
    {
      type: "link",
      href: "#",
      text: "Documents",
      icon: "file",
      badge: undefined,
    },
    {
      type: "link",
      href: "/settings",
      text: "Settings",
      icon: "settings",
      badge: undefined,
    },
  ],
};
