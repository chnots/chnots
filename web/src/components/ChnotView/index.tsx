import { Chnot, ChnotType } from "@/model";
import { v4 as uuid } from "uuid";
import { MarkdownChnotEditor, MarkdownChnotViewer } from "./MarkdownChnot";
import ChnotActionMenu from "../ChnotActionMenu";
import { Tooltip } from "@mui/joy";
import Icon from "../Icon";
import { DomainIcon } from "./DomainSelect";

export interface ChnotViewState {
  isUploadingResource: boolean;
  isRequesting: boolean;
  isComposing: boolean;
}

export enum ChnotViewMode {
  Editor = "editor",
  Preview = "preview",
  Both = "both",
}

export interface ChnotViewProps {
  viewMode: ChnotViewMode;

  className?: string;

  chnot?: Chnot;
}

const ChnotView = ({ chnot: co, className, viewMode }: ChnotViewProps) => {
  const chnot: Chnot = co ?? {
    id: uuid(),
    perm_id: uuid(),
    content: "",
    domain: "public",
    type: ChnotType.MarkdownWithToent,
    insert_time: new Date(),
    update_time: new Date(),
    pinned: false,
    archive_time: undefined,
  };

  const update_time = new Date(chnot.update_time);
  const relativeTimeFormat =
    Date.now() - update_time.getTime() > 7 * 1000 * 60 * 60 * 24
      ? "datetime"
      : "relative";

  return viewMode === ChnotViewMode.Preview ? (
    <div className="group relative flex flex-col justify-start items-start w-full px-4 py-3 mb-2 gap-2 bg-white dark:bg-zinc-800 rounded-lg border border-white dark:border-zinc-800 hover:border-gray-200 dark:hover:border-zinc-700">
      <div className="w-full flex flex-row justify-between items-center gap-2">
        <div className="w-full -mt-0.5 text-xs leading-tight text-gray-400 dark:text-gray-500 select-none">
          <relative-time
            datetime={update_time.toISOString()}
            format={relativeTimeFormat}
            tense="past"
          ></relative-time>
        </div>
        <div className="flex flex-row justify-end items-center select-none shrink-0 gap-2">
          <div className="text-xs ml-1 px-1 italic">
            <DomainIcon
              name={chnot.domain}
              className="w-4 h-auto text-blue-500"
            />
          </div>
          {chnot.pinned && (
            <Tooltip title={"Pinned"} placement="top">
              <Icon.Bookmark className="w-4 h-auto text-amber-500" />
            </Tooltip>
          )}

          <ChnotActionMenu className="-ml-1" chnot={chnot} />
        </div>
      </div>
      <div className="w-full">
      <MarkdownChnotViewer chnot={chnot} />
    </div>
    </div>
  ) : (
    <div
      className={`${
        className ?? ""
      } relative w-full flex flex-col justify-start items-start bg-white dark:bg-zinc-800 p-4 rounded-lg border border-gray-200 dark:border-zinc-700`}
    >
      <MarkdownChnotEditor chnot={chnot} unique={true} />
    </div>
  );
};

export default ChnotView;
