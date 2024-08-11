import { Dropdown, Menu, MenuButton, MenuItem } from "@mui/joy";
import clsx from "clsx";
import useCurrentDomain from "@/hooks/useCurrentUser";
import { useTranslate } from "@/utils/i18n";
import Icon from "./Icon";
import DomainAvatar from "./DomainAvatar";

interface Props {}

const UserBanner = (props: Props) => {
  const t = useTranslate();
  const domain = useCurrentDomain();
  const title = domain ? domain.domain : "Chnots";
  const avatarUrl = domain ? domain.avatarUrl : "/full-logo.webp";

  const handleSignOut = async () => {
    // await authServiceClient.signOut({});
    window.location.href = "/auth";
  };

  return (
    <div className="relative h-auto px-1 shrink-0">
      <Dropdown>
        <MenuButton disabled={!domain} slots={{ root: "div" }}>
          <div
            className={clsx(
              "py-1 my-1 w-auto flex flex-row justify-center align-middle items-center cursor-pointer rounded-2xl border border-transparent text-gray-800 dark:text-gray-400"
            )}
          >
            <DomainAvatar className="shadow shrink-0" avatarUrl={avatarUrl} />
            <span className="ml-2 text-lg font-medium text-slate-800 dark:text-gray-300 shrink truncate">
              {title}
            </span>
          </div>
        </MenuButton>
        <Menu placement="bottom-start" style={{ zIndex: "9999" }}>
          <MenuItem onClick={handleSignOut}>
            <Icon.LogOut className="w-4 h-auto opacity-60" />
            <span className="truncate">{t("common.sign-out")}</span>
          </MenuItem>
        </Menu>
      </Dropdown>
    </div>
  );
};

export default UserBanner;
