import { Chnot } from "@/store/v1/chnot";

import MarkdownPreview from "@uiw/react-markdown-preview";



function splitBodyAndComment(content: string) { 

}

export const MarkdownChnotViewer = ({ chnot }: { chnot: Chnot }) => {
  return (
    <div
      className={`w-100% flex flex-col justify-start items-start text-gray-800 dark:text-gray-400 p-3`}
    >
      <MarkdownPreview
        source={chnot.content}
        style={{
          width: "100%",
        }}
        rehypeRewrite={(node, _index, parent) => {
          if (
            parent &&
            "tagName" in node &&
            "tagName" in parent &&
            node.tagName === "a" &&
            /^h(1|2|3|4|5|6)/.test(parent.tagName)
          ) {
            parent.children = parent.children.slice(1);
          }
        }}
      />
    </div>
  );
};
