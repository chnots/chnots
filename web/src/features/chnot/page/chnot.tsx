import Icon from "@/common/component/icon";
import KButton from "@/common/component/kbutton";
import ChnotList from "@/features/chnot/component/chnot-list";
import { ChnotMarkdownEditor } from "@/features/chnot/component/chnot-markdown-editor";
import ChnotSearch from "@/features/chnot/component/chnot-search";
import { useChnotStore } from "@/store/chnot";
import { useCommonStore } from "@/store/common";
import { useNamespaceStore } from "@/store/namespace";
import { useEffect } from "react";

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
        <div className="shrink-0 border-r kborder flex flex-col w-3/12 bg-secondary">
          <ChnotSearch />
          <div className="overflow-auto h-full bg-background ">
            <ChnotList />
          </div>
        </div>
      )}

      <div className="flex flex-col w-full h-full justify-center items-center p-4">
        <ChnotMarkdownEditor className="w-full max-w-3xl h-full" />
      </div>
    </div>
  );
};

export default ChnotPage;
