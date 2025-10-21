import { TID } from "@/lib/id_util";
import { ChnotMetaKind } from "../vo";
import RichChnot, { PostSaveArg } from "./chnot/rich-chnot";
import MdwtRecord from "./chnot/mdwt";
import { useState } from "react";

const RightSide = ({}: {}) => {};

const Chrome = ({
  onPostSave,
  otid,
  meta,
  readonly,
  content: initialContent,
}: {
  onPostSave: (arg: PostSaveArg) => void;
  otid: TID;
  meta?: ChnotMetaKind;
  readonly?: boolean;
  content?: string;
}) => {
  const [content, setContent] = useState();

  return (
    <div className="grid grid-cols-2">
      <div className="h-full w-full">
        <MdwtRecord
          kindId={meta?.kindId}
          readonly={readonly}
          onPostSave={(arg: PostSaveArg) => {
            onPostSave(arg);
          }}
        />
      </div>
      <div>
        <RichChnot meta={} />
      </div>
    </div>
  );
};

export default RichChnot;
