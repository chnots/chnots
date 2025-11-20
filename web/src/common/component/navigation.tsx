import clsx from 'clsx';
import { NavLink } from 'react-router-dom';

import Icon from './icon';
import { Button } from './ui/button';

import useParamState from '@/hooks/use-param-state';

import { useCommonStore } from '@/common/store';
import { KSpaceSelect } from '@/krate/kspace/component/kspace-select';
import { useKSpaceStore } from '@/krate/kspace/store';
import { useTranslate } from '@/lib/i18n';
import { RoutePaths } from '@/router';

interface NavLinkItem {
  tid: string;
  path: string;
  title: string;
  icon: React.ReactNode;
}

const Navigation = ({
  className,
}: {
  className?: string;
  orientation?: 'vertical' | 'horizontal';
}) => {
  const t = useTranslate();
  const { currentKSpace, selectKSpace } = useKSpaceStore((e) => {
    return {
      currentKSpace: e.currentKSpace,
      selectKSpace: e.selectKSpace,
    };
  });
  const { toggleSidebar } = useCommonStore();
  const [, setKSpaceParam] = useParamState<string>('ns', 'public');

  const chnotNavLink: NavLinkItem = {
    tid: 'header-chnots',
    path: RoutePaths.Chnots,
    title: t('Chnots'),
    icon: <Icon.BrainCircuit className="w-6 h-auto opacity-70 shrink-0" />,
  };
  const llmChatNavLink: NavLinkItem = {
    tid: 'header-llmchat',
    path: RoutePaths.LLMChat,
    title: t('LLM Chat'),
    icon: <Icon.Bot className="w-6 h-auto opacity-70 shrink-0" />,
  };
  /*   const toentNavLink: NavLinkItem = {
    tid: "header-toent",
    path: RoutePaths.Toents,
    title: t("Toents"),
    icon: (
      <Icon.CircleCheckBigIcon className="w-6 h-auto opacity-70 shrink-0" />
    ),
  }; */
  const settingsNavLink: NavLinkItem = {
    tid: 'header-settings',
    path: RoutePaths.Settings,
    title: t('Settings'),
    icon: <Icon.Settings className="w-6 h-auto opacity-70 shrink-0" />,
  };
  const timerNavLink: NavLinkItem = {
    tid: 'header-timer',
    path: RoutePaths.Timer,
    title: t('Timer'),
    icon: <Icon.Timer className="w-6 h-auto opacity-70 shrink-0" />,
  };

  const navLinks: NavLinkItem[] = [chnotNavLink, llmChatNavLink, timerNavLink, settingsNavLink];

  return (
    <div
      className={clsx(
        'h-full overflow-auto flex flex-col items-center z-30 hide-scrollbar kc-basic-with-bdr border-r space-y-2 py-1',
        className,
      )}
    >
      <Button
        onClick={() => {
          toggleSidebar();
        }}
        className="p-2 hover:cursor-pointer rounded-xl"
      >
        <Icon.Sidebar />
      </Button>
      <KSpaceSelect
        onSelect={(ns) => {
          selectKSpace(ns);
          setKSpaceParam(ns);
        }}
        currentKSpace={currentKSpace}
      />
      {navLinks.map((navLink) => (
        <NavLink
          className={({ isActive }) =>
            clsx('p-2 rounded-xl border', isActive ? 'kc-active' : 'border-transparent kc-basic')
          }
          key={navLink.tid}
          to={navLink.path}
          id={navLink.tid}
        >
          <div>{navLink.icon}</div>
        </NavLink>
      ))}
    </div>
  );
};

export default Navigation;
