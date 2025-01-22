import ChnotList from "@/components/chnot/chnot-list";
import { ChnotMarkdownEditor } from "@/components/chnot/chnot-markdown-editor";
import ChnotSearch from "@/components/chnot/chnot-search";
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
    <div className="bg-panel flex h-full max-h-full flex-1 overflow-hidden rounded-md shadow">
      <div className="shrink-0 border-r flex flex-col w-4/12">
        <ChnotSearch />
        <div className="overflow-auto h-full bg-background text-gray-900">
          <ChnotList />
        </div>
      </div>

      <div className="flex-1 p-8 h-full">{<ChnotMarkdownEditor />}</div>
    </div>
  );
};

export default ChnotPage;
