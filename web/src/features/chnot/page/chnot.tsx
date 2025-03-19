import ChnotList from "@/features/chnot/component/chnot-list";
import { ChnotMarkdownEditor } from "@/features/chnot/component/chnot-markdown-editor";
import ChnotSearch from "@/features/chnot/component/chnot-search";
import { useChnotStore } from "@/store/chnot/store";
import { useCommonStore } from "@/store/common";
import { useNamespaceStore } from "@/store/namespace";
import { useEffect } from "react";
import ChnotTagList from "../component/chnot-tag-list";

const ChnotPage = () => {
  const { refreshChnots } = useChnotStore();
  const { currentNamespace } = useNamespaceStore();
  const { showSidebar } = useCommonStore();
  useEffect(() => {
    refreshChnots();
  }, [currentNamespace]);

  return (
    <div className="bg-panel flex h-full max-h-full rounded-md">
      <title>{`Chnots`}</title>
      {showSidebar && (
        <div className="grid grid-cols-2 flex-1">
          <div className="max-h-full h-full overflow-auto">
            <ChnotTagList />
          </div>
          <div className="max-h-full h-full overflow-auto">
            <ChnotList />
          </div>
        </div>
      )}

      <div className="flex-1 justify-center items-center p-4">
        <ChnotMarkdownEditor className="w-full max-w-3xl h-full" />
      </div>
    </div>
  );
};

export default ChnotPage;
