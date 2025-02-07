import { useCallback, useEffect } from "react";

const useDebugChanged = (value: any, title?: string) => {
  useEffect(() => {
    console.info(`>> ${title ?? value} changed`);
  }, [value]);
};

export default useDebugChanged;
