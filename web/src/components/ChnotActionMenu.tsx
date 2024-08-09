/* import { Dropdown, Menu, MenuButton, MenuItem } from "@mui/joy";
import clsx from "clsx";
import { useLocation } from "react-router-dom";
import Icon from "@/components/Icon";
import useNavigateTo from "@/hooks/useNavigateTo";
import { useTranslate } from "@/utils/i18n";
import { Chnot } from "@/model";

interface Props {
  chnot: Chnot;
  className?: string;
  hiddenActions?: ("edit" | "archive" | "delete" | "share" | "pin")[];
}

const ChnotActionMenu = (props: Props) => {
  const { chnot, hiddenActions } = props;
  const t = useTranslate();
  const location = useLocation();
  const navigateTo = useNavigateTo();


  const isInMemoDetailPage = location.pathname.startsWith(`/m/${chnot.id}`);

  const handleTogglePinMemoBtnClick = async () => {
    try {
      if (chnot.pinned_time) {
        await memoStore.updateMemo(
          {
            name: chnot.name,
            pinned: false,
          },
          ["pinned"]
        );
      } else {
        await memoStore.updateMemo(
          {
            name: chnot.name,
            pinned: true,
          },
          ["pinned"]
        );
      }
    } catch (error) {
      // do nth
    }
  };

  const handleEditMemoClick = () => {
    showMemoEditorDialog({
      memoName: chnot.name,
      cacheKey: `${chnot.name}-${chnot.updateTime}`,
    });
  };

  const handleToggleMemoStatusClick = async () => {
    try {
      if (chnot.rowStatus === RowStatus.ARCHIVED) {
        await memoStore.updateMemo(
          {
            name: chnot.name,
            rowStatus: RowStatus.ACTIVE,
          },
          ["row_status"]
        );
        toast(t("message.restored-successfully"));
      } else {
        await memoStore.updateMemo(
          {
            name: chnot.name,
            rowStatus: RowStatus.ARCHIVED,
          },
          ["row_status"]
        );
        toast.success(t("message.archived-successfully"));
      }
    } catch (error: any) {
      console.error(error);
      toast.error(error.response.data.message);
      return;
    }

    if (isInMemoDetailPage) {
      chnot.rowStatus === RowStatus.ARCHIVED
        ? navigateTo("/")
        : navigateTo("/archived");
    }
  };

  const handleDeleteMemoClick = async () => {
    const confirmed = window.confirm(t("memo.delete-confirm"));
    if (confirmed) {
      await memoStore.deleteMemo(chnot.name);
      toast.success(t("message.deleted-successfully"));
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
            {chnot.pinned_time ? (
              <Icon.BookmarkMinus className="w-4 h-auto" />
            ) : (
              <Icon.BookmarkPlus className="w-4 h-auto" />
            )}
            {chnot.pinned_time ? t("common.unpin") : t("common.pin")}
          </MenuItem>
        )}
        {!hiddenActions?.includes("edit") && (
          <MenuItem onClick={handleEditMemoClick}>
            <Icon.Edit3 className="w-4 h-auto" />
            {t("common.edit")}
          </MenuItem>
        )}
        <MenuItem color="warning" onClick={handleToggleMemoStatusClick}>
          {chnot.rowStatus === RowStatus.ARCHIVED ? (
            <Icon.ArchiveRestore className="w-4 h-auto" />
          ) : (
            <Icon.Archive className="w-4 h-auto" />
          )}
          {chnot.rowStatus === RowStatus.ARCHIVED
            ? t("common.restore")
            : t("common.archive")}
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
 */