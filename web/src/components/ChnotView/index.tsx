import { Chnot, ChnotType } from "@/model";
import { v4 as uuid } from "uuid";
import { MarkdownChnotEditor, MarkdownChnotViewer } from "./MarkdownChnot";

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
  const chnot = co ?? {
    id: uuid(),
    perm_id: uuid(),
    content: "",
    domain: "public",
    type: ChnotType.MarkdownWithToent,
    insert_time: new Date(),
    update_time: new Date(),
  };

  return viewMode === ChnotViewMode.Preview ? (
    <div className="group relative flex flex-col justify-start items-start w-full px-4 py-3 mb-2 gap-2 bg-white dark:bg-zinc-800 rounded-lg border border-white dark:border-zinc-800 hover:border-gray-200 dark:hover:border-zinc-700">
      <MarkdownChnotViewer chnot={chnot} />
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
