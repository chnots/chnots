import MarkdownEditor from "@uiw/react-markdown-editor";
import { useState } from "react";
import { Chnot } from "../../model";

export interface UiwMdEditorProps {
  className?: string;

  chnot?: Chnot;
}

export const UiwMdEditor = (props: UiwMdEditorProps) => {
  const [value, setValue] = useState(props.chnot?.content);

  return (
    <div>
      <MarkdownEditor value={value} onChange={setValue} />
    </div>
  );
};
