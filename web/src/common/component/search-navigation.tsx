import clsx from "clsx";
import { useCommonStore } from "@/common/store";
import Icon from "./icon";

const SearchButton = () => {
  const { toggleNaviSearch, getNaviSearch } = useCommonStore();

  return (
    <button
      type="button"
      onClick={() => {
        toggleNaviSearch();
      }}
      className={clsx(
        "rounded-xl border p-2",
        getNaviSearch() ? "bg-white border-gray-400" : "border-gray-100",
      )}
    >
      <Icon.Search />
    </button>
  );
};

export default SearchButton;
