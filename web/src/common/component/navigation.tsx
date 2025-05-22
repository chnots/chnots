import clsx from "clsx";
import { NavLink } from "react-router-dom";
import { useTranslate } from "@/utils/i18n";
import Icon from "./icon";
import { KSpaceSelect } from "./kspace-select";
import { RoutePaths } from "@/router";
import { useKSpaceStore } from "@/store/kspace";
import KButton from "./kbutton";
import { useCommonStore } from "@/store/common";
import useParamState from "@/hooks/use-param-state";

interface NavLinkItem {
  id: string;
  path: string;
  title: string;
  icon: React.ReactNode;
}

const Navigation = ({
  className,
}: {
  className?: string;
  orientation?: "vertical" | "horizontal";
}) => {
  const t = useTranslate();
  const { currentKSpace, changeKSpace } = useKSpaceStore();
  const { toggleSidebar } = useCommonStore();
  const [, setKSpaceParam] = useParamState<string>("ns", "public");

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
  /*   const toentNavLink: NavLinkItem = {
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
  }; */
  const timerNavLink: NavLinkItem = {
    id: "header-timer",
    path: RoutePaths.Timer,
    title: t("Timer"),
    icon: <Icon.Timer className="w-6 h-auto opacity-70 shrink-0" />,
  };

  const navLinks: NavLinkItem[] = [chnotNavLink, llmChatNavLink, timerNavLink];

  return (
    <div
      className={clsx(
        "h-full overflow-auto flex flex-col items-center z-30 hide-scrollbar kc-basic-with-bdr border-r space-y-2 py-1",
        className
      )}
    >
      <KButton
        onClick={() => {
          toggleSidebar();
        }}
        className="p-2 hover:cursor-pointer rounded-xl"
      >
        <Icon.Sidebar />
      </KButton>
      <KSpaceSelect
        onSelect={(ns) => {
          changeKSpace(ns);
          setKSpaceParam(ns);
        }}
        currentKSpace={currentKSpace.name}
        menuClassName="p-2 border  hover:cursor-pointer rounded-xl"
      />
      {navLinks.map((navLink) => (
        <NavLink
          className={({ isActive }) =>
            clsx(
              "p-2 rounded-xl border",
              isActive ? "kc-active" : "border-transparent kc-basic"
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
  );
};

export default Navigation;
