import { useChnotStore } from "@/store/v1/chnot";
import DebounceInput from "./DebounceInput";

const SearchPanel = () => {
  const chnotStore = useChnotStore();

  return (
    <div className="h-full">
      <DebounceInput
        handleDebounce={function (value: string): void {
          chnotStore.changeKeyword(value);
        }}
        debounceTimeout={300}
      ></DebounceInput>
    </div>
  );
};

export default SearchPanel;
