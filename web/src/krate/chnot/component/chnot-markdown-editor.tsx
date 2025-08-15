import { MdwtEditorMemo } from "@/common/component/codemirror/mdwt-editor";
import { chnotTagNames, toentGuess } from "@/krate/chnot/service";
import { CompletionContext, CompletionResult } from "@codemirror/autocomplete";

const chnotCompletions = async (
  context: CompletionContext,
): Promise<CompletionResult | null> => {
  const word = context.matchBefore(/#[^# ]*|^#* \[|^[ ]*- \[|\[\[/);
  let options;
  if (!word || (word?.from == word?.to && !context.explicit)) {
    return null;
  } else if (word.text.startsWith("#")) {
    options = (
      await chnotTagNames({
        query: word.text,
        start_index: 0,
        page_size: 20,
      })
    ).data.map((name) => {
      return { label: name, type: "hashtag" };
    });
  } else if (word.text.startsWith("[[")) {
    options = [{ label: `[[backlink-ph]]`, type: "backlink" }];
  } else if (word.text.includes("# [") || word.text.includes("- [")) {
    options = (
      await toentGuess({ input: word.text.replace(/.*\[/, "") })
    ).toents.map((toent) => {
      return { label: `{${toent.event}}`, type: "toent" };
    });
  } else {
    return null;
  }

  options.sort((e1, e2) => e1.label.length - e2.label.length);

  return {
    from: word.from,
    options: options,
    filter: false,
  };
};

const MdwtEditor = ({
  content,
  onContentChange,
  height,
  foldGutter,
}: {
  content?: string;
  onContentChange: (content: string) => void;
  height?: number;
  foldGutter: boolean;
}) => {
  return (
    <MdwtEditorMemo
      content={content}
      onContentChange={onContentChange}
      autoCompletion={chnotCompletions}
      height={height}
      foldGutter={foldGutter}
    />
  );
};

export default MdwtEditor;
