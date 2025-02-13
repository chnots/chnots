import React from "react";

const useDebounce = (fn: (...args: any[]) => void, duration?: number) => {
  const timeoutRef = React.useRef(0);

  return React.useCallback(
    (...args: unknown[]) => {
      window.clearTimeout(timeoutRef.current);
      timeoutRef.current = window.setTimeout(() => {
        fn(...args);
      }, duration ?? 1000);
    },

    [duration, fn]
  );
};

export default useDebounce;
