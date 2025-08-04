import KSpaceSettings from "@/krate/kspace/component/settings";
import { EndpointSettings } from "@/krate/sync/component/settings";
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
import React, { useState } from "react";
import Icon from "../component/icon";
import { Button } from "../component/ui/button";
import { Toggle } from "../component/ui/toggle";
import { useCommonStore } from "../store";
import { NavLink } from "react-router-dom";
import { RoutePaths } from "@/router";

enum SettingsEnum {
  Endpoint = "Endpoint Settings",
  KSpace = "KSpace Settings",
}

const SettingsItem = ({
  children,
  ...props
}: {
  children: React.ReactNode;
} & React.ComponentProps<"li">) => {
  return (
    <SidebarMenuItem
      className="p-2 m-2 list-none flex align-middle items-center space-x-2 h-4 w-full"
      {...props}
    >
      <Button variant={"link"} className="w-full">
        {children}
      </Button>
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
        <SidebarHeader className="w-full justify-between items-center flex flex-row p-4">
          <h1 className="text-2xl">Settings</h1>
          <NavLink to={RoutePaths.Chnots} id={"chnot"}>
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
                >
                  <Icon.Earth />
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
                >
                  <Icon.Network />
                  <span>Endpoints</span>
                </SettingsItem>
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
      </Sidebar>
      <SidebarInset className="w-full ">
        <div className="w-full m-4 items-center flex flex-row">
          <SidebarTrigger />
          <h2 className="text-xl">{settingsEnum}</h2>
        </div>
        <div className="m-8">
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
