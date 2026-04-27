import type { TID } from "@/lib/id_util";
import MdwtChnot from "./mdwt";
import type { PostSaveArg } from "./types";

const RichMdwt = ({
  onPostSave,
  otid,
  readonly,
  content: initialContent,
  disableHeaderActions,
}: {
  onPostSave: (arg: PostSaveArg) => Promise<void>;
  otid: TID;
  readonly?: boolean;
  content?: string;
  disableHeaderActions?: boolean;
}) => {
  return (
    <div className="w-full p-1 h-full flex flex-col divide-y max-w-4xl">
      <div className="min-h-0 overflow-hidden flex-1">
        <MdwtChnot
          otid={otid}
          readonly={readonly}
          onPostSave={onPostSave}
          content={initialContent}
          fillParentHeight={true}
          disableHeaderActions={disableHeaderActions}
          fullscreen={false}
          onSetFullscreen={() => {}}
        />
      </div>
    </div>
  );
};

export default RichMdwt;
