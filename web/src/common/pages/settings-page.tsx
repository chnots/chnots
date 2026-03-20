import clsx from "clsx";
import { Brain, Earth, Network, RefreshCw } from "lucide-react";
import type React from "react";
import { useState } from "react";
import { NavLink } from "react-router-dom";
import KSpaceSettings from "@/krate/kspace/component/settings";
import FixDb from "@/krate/mdwt/component/fix-db";
import { EndpointSettings } from "@/krate/sync/component/settings";
import { RoutePaths } from "@/router";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
  SidebarProvider,
  SidebarSeparator,
  SidebarTrigger,
} from "../component/ui/sidebar";

enum SettingsEnum {
  Endpoint = "Endpoint Settings",
  KSpace = "KSpace Settings",
  FixDb = "Fix Database",
}

const SettingsItem = ({
  children,
  focused,
  ...props
}: {
  children: React.ReactNode;
  focused: boolean;
} & React.ComponentProps<"li">) => {
  return (
    <SidebarMenuItem
      className={clsx(
        "list-none flex items-center space-x-2 w-full text-sm hover:cursor-pointer hover:bg-background border rounded-lg px-2 py-1",
        focused ? "bg-background" : "border-transparent",
      )}
      {...props}
    >
      {children}
    </SidebarMenuItem>
  );
};

const Settings = () => {
  const [settingsEnum, setSettingsEnum] = useState<SettingsEnum>(
    SettingsEnum.KSpace,
  );
  return (
    <SidebarProvider>
      <Sidebar variant="inset">
        <SidebarHeader className="text-sm">
          <div className="flex justify-between items-center">
            <div className="flex items-center space-x-1">
              <div className="w-8 h-8 flex items-center justify-center" />
            </div>
            <NavLink to={RoutePaths.Chnots} id={"chnot"}>
              <Brain className="w-4 h-4" />
            </NavLink>
          </div>
        </SidebarHeader>
        <SidebarSeparator className="mx-0" />
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupLabel>Function</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                <SettingsItem
                  onClick={() => setSettingsEnum(SettingsEnum.KSpace)}
                  focused={settingsEnum === SettingsEnum.KSpace}
                >
                  <Earth className="w-4 h-4" />
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
                  <Network className="w-4 h-4" />
                  <span>Endpoints</span>
                </SettingsItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
          <SidebarGroup>
            <SidebarGroupLabel>System</SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                <SettingsItem
                  onClick={() => setSettingsEnum(SettingsEnum.FixDb)}
                  focused={settingsEnum === SettingsEnum.FixDb}
                >
                  <RefreshCw className="w-4 h-4" />
                  <span>Fix Database</span>
                </SettingsItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>
      <main className="flex h-svh w-full flex-col overflow-hidden border-l bg-background">
        <header className="flex h-14 items-center gap-2 border-b px-4">
          <SidebarTrigger />
          <h2 className="text-base font-medium">{settingsEnum}</h2>
        </header>
        <div className="flex-1 overflow-auto p-4 sm:p-6">
          <div className="mx-auto w-full max-w-4xl">
            {settingsEnum === SettingsEnum.KSpace ? (
              <KSpaceSettings />
            ) : settingsEnum === SettingsEnum.Endpoint ? (
              <EndpointSettings />
            ) : (
              settingsEnum === SettingsEnum.FixDb && <FixDb />
            )}
          </div>
        </div>
      </main>
    </SidebarProvider>
  );
};

export default Settings;
