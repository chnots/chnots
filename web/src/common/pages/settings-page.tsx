import { useState } from 'react';
import clsx from 'clsx';
import { NavLink } from 'react-router-dom';

import Icon from '../component/icon';
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuItem,
  SidebarProvider,
  SidebarTrigger,
} from '../component/ui/sidebar';

import type React from 'react';
import KSpaceSettings from '@/krate/kspace/component/settings';
import { EndpointSettings } from '@/krate/sync/component/settings';
import { RoutePaths } from '@/router';

enum SettingsEnum {
  Endpoint = 'Endpoint Settings',
  KSpace = 'KSpace Settings',
}

const SettingsItem = ({
  children,
  focused,
  ...props
}: {
  children: React.ReactNode;
  focused: boolean;
} & React.ComponentProps<'li'>) => {
  return (
    <SidebarMenuItem
      className={clsx(
        'list-none flex items-center space-x-2 w-full text-sm hover:cursor-pointer hover:bg-background border rounded-lg px-2 py-1',
        focused ? 'bg-background' : 'border-transparent',
      )}
      {...props}
    >
      {children}
    </SidebarMenuItem>
  );
};

const Settings = () => {
  const [settingsEnum, setSettingsEnum] = useState<SettingsEnum>(SettingsEnum.KSpace);
  return (
    <SidebarProvider>
      <Sidebar>
        <SidebarHeader className="w-full justify-between items-center flex flex-row p-4">
          <div></div>
          <NavLink to={RoutePaths.Chnots} id={'chnot'}>
            <Icon.Brain className="w-4 h-4" />
          </NavLink>
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupLabel>Function</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                <SettingsItem
                  onClick={() => setSettingsEnum(SettingsEnum.KSpace)}
                  focused={settingsEnum === SettingsEnum.KSpace}
                >
                  <Icon.Earth className="w-4 h-4" />
                  <span>KSpace</span>
                </SettingsItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
          <SidebarGroup>
            <SidebarGroupLabel>Sync</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                <SettingsItem
                  onClick={() => setSettingsEnum(SettingsEnum.Endpoint)}
                  focused={settingsEnum === SettingsEnum.Endpoint}
                >
                  <Icon.Network className="w-4 h-4" />
                  <span>Endpoints</span>
                </SettingsItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>
      <SidebarInset className="flex flex-col w-full items-center">
        <div className="w-full m-4 items-center flex pl-10">
          <SidebarTrigger />
          <h2 className="text-xl">{settingsEnum}</h2>
        </div>
        <div className="m-8 max-w-4xl">
          {settingsEnum === SettingsEnum.KSpace ? (
            <KSpaceSettings />
          ) : settingsEnum === SettingsEnum.Endpoint ? (
            <EndpointSettings />
          ) : (
            <></>
          )}
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
};

export default Settings;
