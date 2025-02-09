import { useChnotStore } from "@/store/chnot";
import { useCommonStore } from "@/store/common";

function ChnotSearch() {
  const commonStore = useCommonStore();
  const chnotStore = useChnotStore();

  return commonStore.getNaviSearch() && (
    <div className="w-full p-2 bg-transparent rounded border border-gray-200">
      <input
        type="text"
        className="bg-transparent w-full h-full outline-none"
        placeholder="search"
        onChange={(value) => chnotStore.changeKeyword(value.target.value)}
      />
    </div>
  );
}

export default ChnotSearch;
