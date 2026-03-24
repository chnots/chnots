import React, { useCallback, useEffect, useRef } from "react";
import type { TimeoutType } from "@/lib/types";

type DebouncedFunction<T extends any[]> = (...args: T) => void;

interface UseDebounceOptions {
  duration?: number;
  executeOnUnmount?: boolean;
}

const useDebounce = <T extends any[]>(
  fn: DebouncedFunction<T>,
  options: UseDebounceOptions = {},
): DebouncedFunction<T> => {
  const { duration = 1000, executeOnUnmount = false } = options;

  const timeoutRef = useRef<TimeoutType | null>(null);
  const argsRef = useRef<T | null>(null);
  const fnRef = useRef(fn);

  // Keep fnRef up to date
  useEffect(() => {
    fnRef.current = fn;
  }, [fn]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }

      if (executeOnUnmount && argsRef.current) {
        fnRef.current(...argsRef.current);
      }
    };
  }, [executeOnUnmount]);

  const debouncedFunction = useCallback<DebouncedFunction<T>>(
    (...args: T) => {
      argsRef.current = args;

      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }

      timeoutRef.current = setTimeout(() => {
        fnRef.current(...args);
        argsRef.current = null;
      }, duration);
    },
    [duration],
  );

  return debouncedFunction;
};

export default useDebounce;
