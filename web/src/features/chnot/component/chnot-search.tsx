import { useChnotStore } from "@/store/chnot/store";
import { useCommonStore } from "@/store/common";

function ChnotSearch() {
  const { getNaviSearch } = useCommonStore();
  const { changeKeyword } = useChnotStore();

  return (
    getNaviSearch() && (
      <div className="w-full p-2 bg-transparent rounded border border-gray-200">
        <input
          type="text"
          className="bg-transparent w-full h-full outline-none"
          placeholder="search"
          onChange={(value) => changeKeyword(value.target.value)}
        />
      </div>
    )
  );
}

export default ChnotSearch;
