import clsx from "clsx";
import ChnotList from "@/components/ChnotList";
import ChnotRingView, { ChnotRingViewMode } from "@/components/ChnotRing";
import { useCommonStore } from "@/store/v1/common";
import SearchPanel from "@/components/SearchPanel";

const Chnots = () => {
  const commonStore = useCommonStore();

  return (
    <div
      className={clsx(
        "w-full flex flex-row justify-center items-start px-4 sm:px-6 gap-4 m-4"
      )}
    >
      {commonStore.getNaviSearch() && (
        <div>
          <SearchPanel />
        </div>
      )}

      <div className={clsx("w-full max-w-2xl")}>
        <ChnotRingView
          className="mb-2"
          viewMode={ChnotRingViewMode.Editor}
          createInput={true}
        />
        <ChnotList />
      </div>
    </div>
  );
};

export default Chnots;
