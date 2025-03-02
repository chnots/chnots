import clsx from "clsx";
import { NavLink } from "react-router-dom";
import { useTranslate } from "@/utils/i18n";
import Icon from "./icon";
import { NamespaceSelect } from "./namespace-select";
import SearchButton from "./search-navigation";
import { RoutePaths } from "@/router";
import { useNamespaceStore } from "@/store/namespace";
import KButton from "./kbutton";
import { useCommonStore } from "@/store/common";

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
  const { currentNamespace, changeNamespace } = useNamespaceStore();
  const { toggleSidebar } = useCommonStore();

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
        "w-full overflow-auto flex flex-row items-center z-30 hide-scrollbar bg-secondary border-b kborder space-x-4 py-1 pl-5",
        className
      )}
    >
      <KButton
        onClick={() => {
          toggleSidebar();
        }}
      >
        <Icon.List />
      </KButton>
      <NamespaceSelect
        onSelect={(ns) => {
          changeNamespace(ns);
        }}
        currentNamespace={currentNamespace.name}
      />
      <SearchButton />
      {navLinks.map((navLink) => (
        <NavLink
          className={({ isActive }) =>
            clsx("p-2", isActive ? "kbutton-focused" : "kbutton-muted")
          }
          key={navLink.id}
          to={navLink.path}
          id={navLink.id}
        >
          <div>{navLink.icon}</div>
        </NavLink>
      ))}
    </header>
  );
};

export default Navigation;
