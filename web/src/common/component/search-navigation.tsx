import Icon from "./icon";
import { useCommonStore } from "@/common/store";
import clsx from "clsx";

const SearchButton = () => {
  const { toggleNaviSearch, getNaviSearch } = useCommonStore();

  return (
    <button
      onClick={function () {
        toggleNaviSearch();
      }}
      className={clsx(
        "rounded-xl border p-2",
        getNaviSearch() ? "bg-white border-gray-400" : "border-gray-100"
      )}
    >
      <Icon.Search />
    </button>
  );
};

export default SearchButton;
