import ChnotList from "@/features/chnot/component/chnot-list";
import { ChnotMarkdownEditor } from "@/features/chnot/component/chnot-markdown-editor";
import ChnotSearch from "@/features/chnot/component/chnot-search";
import { useChnotStore } from "@/store/chnot";
import { useNamespaceStore } from "@/store/namespace";
import { useEffect } from "react";

const ChnotPage = () => {
  const { refreshChnots } = useChnotStore();
  const { currentNamespace } = useNamespaceStore();

  useEffect(() => {
    refreshChnots();
  }, [currentNamespace]);

  return (
    <div className="bg-panel flex h-full max-h-full flex-1 overflow-hidden rounded-md">
      <div className="shrink-0 border-r kborder flex flex-col w-3/12 bg-secondary">
        <ChnotSearch />
        <div className="overflow-auto h-full bg-background text-gray-900">
          <ChnotList />
        </div>
      </div>

      <div className="flex flex-row p-8 h-full justify-center w-full">
        <div className="w-3xl max-w-3xl">
          <ChnotMarkdownEditor />
        </div>
      </div>
    </div>
  );
};

export default ChnotPage;
