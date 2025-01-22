import { useCallback } from "react";

const useDebugChanged = (title: string, value: any) => {
  useCallback(() => {
    console.debug(`>> ${title} changed`);
  }, [value]);
};

export default useDebugChanged;
