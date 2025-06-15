import { useChnotStore } from "@/krate/chnot/store";
import DebounceInput from "./debounce-input";

const SearchPanel = () => {
  const { changeKeyword } = useChnotStore();

  return (
    <div className="h-full">
      <DebounceInput
        handleDebounce={function (value: string): void {
          changeKeyword(value);
        }}
        debounceTimeout={300}
      ></DebounceInput>
    </div>
  );
};

export default SearchPanel;
