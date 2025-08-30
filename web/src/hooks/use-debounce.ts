import React, { useEffect } from "react";

const useDebounce = (
  fn: (...args: any[]) => void,
  duration?: number,
  executeOnExit: boolean = false,
) => {
  const timeoutRef = React.useRef(0);
  const argsRef = React.useRef<any[]>(undefined);

  useEffect(() => {
    return () => {
      if (argsRef.current && executeOnExit) {
        window.clearTimeout(timeoutRef.current);
        fn(...argsRef.current);
        argsRef.current = undefined;
      }
    };
  }, [argsRef, executeOnExit, timeoutRef]);

  return React.useCallback(
    (...args: unknown[]) => {
      argsRef.current = args;
      window.clearTimeout(timeoutRef.current);
      timeoutRef.current = window.setTimeout(() => {
        fn(...args);
        // do nothing when exited if we invoke it.
        argsRef.current = undefined;
      }, duration ?? 1000);
    },

    [duration, fn],
  );
};

export default useDebounce;
