import { useCallback } from "react";

const useDebugChanged = (value: any) => {
  useCallback(() => {
    console.error(`>> ${value} changed`);
  }, [value]);
};

export default useDebugChanged;
