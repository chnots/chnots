import { useEffect } from "react";

const useDebugChanged = (value: any, title?: string) => {
  useEffect(() => {
    console.log("> changed,", title, value);
  }, [value]);
};

export default useDebugChanged;
