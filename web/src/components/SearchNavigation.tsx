import Icon from "./Icon";
import { useCommonStore } from "@/store/v1/common";
import clsx from "clsx";

const SearchButton = () => {
  const { toggleNaviSearch, getNaviSearch } = useCommonStore();

  return (
    <div className="mt-2">
      <button
        onClick={function () {
          toggleNaviSearch();
        }}
        className={clsx(
          "rounded-2xl border p-1.5",
          getNaviSearch() ? "bg-white border-gray-400" : "border-gray-100"
        )}
      >
        <Icon.Search />
      </button>
    </div>
  );
};

export default SearchButton;
