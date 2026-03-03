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
  SidebarInset,
  SidebarMenu,
  SidebarMenuItem,
  SidebarProvider,
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
      <Sidebar>
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
            settingsEnum === SettingsEnum.FixDb && <FixDb />
          )}
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
};

export default Settings;
