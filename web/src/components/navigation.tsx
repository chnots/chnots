import clsx from "clsx";
import { NavLink } from "react-router-dom";
import { useTranslate } from "@/utils/i18n";
import Icon from "./icon";
import { NamespaceSelect } from "./namespace-select";
import SearchButton from "./search-navigation";
import { RoutePaths } from "@/router";

interface NavLinkItem {
  id: string;
  path: string;
  title: string;
  icon: React.ReactNode;
}

interface Props {
  className?: string;
}

const Navigation = (props: Props) => {
  const { className } = props;
  const t = useTranslate();

  const chnotNavLink: NavLinkItem = {
    id: "header-chnots",
    path: RoutePaths.Chnots,
    title: t("Chnots"),
    icon: <Icon.BrainCircuit className="w-6 h-auto opacity-70 shrink-0" />,
  };
  const llmChatNavLink: NavLinkItem = {
    id: "header-llmchat",
    path: RoutePaths.LLMChat,
    title: t("LLM Chat"),
    icon: <Icon.Bot className="w-6 h-auto opacity-70 shrink-0" />,
  };
  const toentNavLink: NavLinkItem = {
    id: "header-toent",
    path: RoutePaths.Toents,
    title: t("Toents"),
    icon: (
      <Icon.CircleCheckBigIcon className="w-6 h-auto opacity-70 shrink-0" />
    ),
  };
  const settingsNavLink: NavLinkItem = {
    id: "header-settings",
    path: RoutePaths.Settings,
    title: t("Settings"),
    icon: <Icon.Settings className="w-6 h-auto opacity-70 shrink-0" />,
  };

  const navLinks: NavLinkItem[] = [
    chnotNavLink,
    llmChatNavLink,
    toentNavLink,
    settingsNavLink,
  ];

  return (
    <header
      className={clsx(
        "w-full h-full overflow-auto flex flex-col justify-center items-center py-4 md:pt-6 z-30 hide-scrollbar",
        className
      )}
    >
      <NamespaceSelect />
      <SearchButton />
      <div className="w-full px-1 py-2 flex flex-col justify-center items-center shrink-0 space-y-2">
        {navLinks.map((navLink) => (
          <NavLink
            className={({ isActive }) =>
              clsx(
                "px-2 py-2 rounded-2xl border flex flex-row items-center text-lg text-gray-800 dark:text-gray-400 hover:bg-white hover:border-gray-200 dark:hover:border-zinc-700 dark:hover:bg-zinc-800",
                isActive
                  ? "bg-white drop-shadow-sm dark:bg-zinc-800 border-gray-200 dark:border-zinc-700"
                  : "border-transparent"
              )
            }
            key={navLink.id}
            to={navLink.path}
            id={navLink.id}
          >
            <div>{navLink.icon}</div>
          </NavLink>
        ))}
      </div>
    </header>
  );
};

export default Navigation;
