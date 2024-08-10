import { Dropdown, Menu, MenuButton, MenuItem } from "@mui/joy";
import clsx from "clsx";
import { useLocation } from "react-router-dom";
import Icon from "@/components/Icon";
import useNavigateTo from "@/hooks/useNavigateTo";
import { useTranslate } from "@/utils/i18n";
import { Chnot } from "@/model";
import { useChnotStore } from "@/store/v1/chnot";

interface Props {
  chnot: Chnot;
  className?: string;
  hiddenActions?: ("edit" | "archive" | "delete" | "share" | "pin")[];
  changeMode: () => void;
}

const ChnotActionMenu = (props: Props) => {
  const { chnot, hiddenActions, changeMode } = props;
  const t = useTranslate();
  const location = useLocation();
  const navigateTo = useNavigateTo();

  const chnotStore = useChnotStore();

  const isInMemoDetailPage = location.pathname.startsWith(`/m/${chnot.id}`);

  const handleTogglePinMemoBtnClick = async () => {
    try {
      await chnotStore.updateChnot({
        chnot_id: chnot.id,
        pinned: !chnot.pinned,
        update_time: false,
      });
    } catch (error) {
      // do nth
    }
  };

  const handleEditMemoClick = () => {
    changeMode();
  };

  const handleToggleMemoStatusClick = async () => {
    try {
      await chnotStore.updateChnot({
        chnot_id: chnot.id,
        archive: !chnot.archive_time,
        update_time: false,
      });
    } catch (error) {
      // do nth
    }
  };

  const handleDeleteMemoClick = async () => {
    const confirmed = window.confirm(t("memo.delete-confirm"));
    if (confirmed) {
      await chnotStore.deleteChnot({
        chnot_id: chnot.id,
        logic: true,
      });
      if (isInMemoDetailPage) {
        navigateTo("/");
      }
    }
  };

  return (
    <Dropdown>
      <MenuButton slots={{ root: "div" }}>
        <span
          className={clsx(
            "flex justify-center items-center rounded-full hover:opacity-70",
            props.className
          )}
        >
          <Icon.MoreVertical className="w-4 h-4 mx-auto text-gray-500 dark:text-gray-400" />
        </span>
      </MenuButton>
      <Menu className="text-sm" size="sm" placement="bottom-end">
        {!hiddenActions?.includes("pin") && (
          <MenuItem onClick={handleTogglePinMemoBtnClick}>
            {chnot.pinned ? (
              <Icon.BookmarkMinus className="w-4 h-auto" />
            ) : (
              <Icon.BookmarkPlus className="w-4 h-auto" />
            )}
            {chnot.pinned ? t("common.unpin") : t("common.pin")}
          </MenuItem>
        )}
        {!hiddenActions?.includes("edit") && (
          <MenuItem onClick={handleEditMemoClick}>
            <Icon.Edit3 className="w-4 h-auto" />
            {t("common.edit")}
          </MenuItem>
        )}
        <MenuItem color="warning" onClick={handleToggleMemoStatusClick}>
          {chnot.archive_time ? (
            <Icon.ArchiveRestore className="w-4 h-auto" />
          ) : (
            <Icon.Archive className="w-4 h-auto" />
          )}
          {chnot.archive_time ? t("common.restore") : t("common.archive")}
        </MenuItem>
        <MenuItem color="danger" onClick={handleDeleteMemoClick}>
          <Icon.Trash className="w-4 h-auto" />
          {t("common.delete")}
        </MenuItem>
      </Menu>
    </Dropdown>
  );
};

export default ChnotActionMenu;
