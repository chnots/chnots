import MarkdownEditor from "@uiw/react-markdown-editor";
import { Button } from "@mui/joy";
import { v4 as uuid } from "uuid";
import { useState } from "react";
import { Chnot, ChnotType } from "@/model";
import Icon from "../Icon";
import { overwriteChnot } from "@/helpers/data-agent";

export interface MarkdownChnotProps {
  showPreview?: boolean;
  showEditor?: boolean;
  className?: string;

  chnot?: Chnot;
}

interface EditorState {
  isUploadingResource: boolean;
  isRequesting: boolean;
  isComposing: boolean;
}

export const MarkdownChnot = (props: MarkdownChnotProps) => {
  const chnot = props.chnot ?? {
    id: uuid(),
    perm_id: uuid(),
    content: "",
    domain: "public",
    type: ChnotType.MarkdownWithToent,
    insert_time: new Date(),
    update_time: new Date(),
  };

  const [content, setContent] = useState(props.chnot?.content);

  const [state, setState] = useState<EditorState>({
    isUploadingResource: false,
    isRequesting: false,
    isComposing: false,
  });

  const handleSaveBtnClick = async () => {
    if (state.isRequesting) {
      return;
    }

    setState((state) => {
      return {
        ...state,
        isRequesting: true,
      };
    });

    const toSave = { ...chnot, id: uuid(), content: content ?? "" };

    const req = { chnot: toSave };
    try {
      await overwriteChnot(req);
      setContent("");
    } finally {
      setState((state) => {
        return {
          ...state,
          isRequesting: false,
        };
      });
    }
  };

  return (
    <div>
      <MarkdownEditor
        showToolbar={false}
        visible={false}
        enablePreview={false}
        value={content}
        onChange={setContent}
      />

      <div className="shrink-0 flex flex-row justify-end items-center">
        <Button
          className="!font-normal"
          loading={state.isRequesting}
          endDecorator={<Icon.Send className="w-4 h-auto" />}
          onClick={handleSaveBtnClick}
          disabled={!content}
        >
          Save
        </Button>
      </div>
    </div>
  );
};
