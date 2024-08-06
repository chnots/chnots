import { Tooltip } from "@mui/joy";
import clsx from "clsx";
import { NavLink } from "react-router-dom";
import { Routes } from "@/router";
import { useTranslate } from "@/utils/i18n";
import Icon from "./Icon";
import UserBanner from "./UserBanner";

interface NavLinkItem {
  id: string;
  path: string;
  title: string;
  icon: React.ReactNode;
}

interface Props {
  collapsed?: boolean;
  className?: string;
}

const Navigation = (props: Props) => {
  const { collapsed, className } = props;
  const t = useTranslate();

  const chnotNavLink: NavLinkItem = {
    id: "header-home",
    path: Routes.Chnots,
    title: t("common.home"),
    icon: <Icon.Home className="w-6 h-auto opacity-70 shrink-0" />,
  };
  const toentNavLink: NavLinkItem = {
    id: "header-timeline",
    path: Routes.Toents,
    title: t("timeline.title"),
    icon: <Icon.GanttChartSquare className="w-6 h-auto opacity-70 shrink-0" />,
  };

  const navLinks: NavLinkItem[] = [chnotNavLink, toentNavLink];

  return (
    <header
      className={clsx(
        "w-full h-full overflow-auto flex flex-col justify-start items-start py-4 md:pt-6 z-30 hide-scrollbar",
        className
      )}
    >
      <UserBanner collapsed={collapsed} />
      <div className="w-full px-1 py-2 flex flex-col justify-start items-start shrink-0 space-y-2">
        {navLinks.map((navLink) => (
          <NavLink
            className={({ isActive }) =>
              clsx(
                "px-2 py-2 rounded-2xl border flex flex-row items-center text-lg text-gray-800 dark:text-gray-400 hover:bg-white hover:border-gray-200 dark:hover:border-zinc-700 dark:hover:bg-zinc-800",
                collapsed ? "" : "w-full px-4",
                isActive
                  ? "bg-white drop-shadow-sm dark:bg-zinc-800 border-gray-200 dark:border-zinc-700"
                  : "border-transparent"
              )
            }
            key={navLink.id}
            to={navLink.path}
            id={navLink.id}
            unstable_viewTransition
          >
            {props.collapsed ? (
              <Tooltip title={navLink.title} placement="right" arrow>
                <div>{navLink.icon}</div>
              </Tooltip>
            ) : (
              navLink.icon
            )}
            {!props.collapsed && (
              <span className="ml-3 truncate">{navLink.title}</span>
            )}
          </NavLink>
        ))}
      </div>
    </header>
  );
};

export default Navigation;
