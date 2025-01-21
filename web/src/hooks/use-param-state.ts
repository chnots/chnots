// Adapted from https://dev.to/mr_mornin_star/custom-react-hook-to-sync-state-with-the-url-4b6p

import { useCallback, useState } from "react";
import { useSearchParams } from "react-router-dom";

/**
 * A custom hook that syncs state with a URL search parameter.
 * Supports string, number, boolean, and object values.
 * @param key The search parameter key to sync with.
 * @param defaultValue The default value for the state.
 * @returns A stateful value, and a function to update it.
 */
function useParamState<T extends string | number | boolean>(
  key: string,
  defaultValue: T
): [T, (newValue: T) => void] {
  const [searchParams, setSearchParams] = useSearchParams();
  const paramValue = searchParams.get(key);

  // Initialize state based on the search parameter or default value
  const [state, setState] = useState<T>(() => {
    if (paramValue === null) return defaultValue;
    return paramValue as T;
  });

  // Update the state and search params
  const setParamState = useCallback(
    (newValue: T) => {
      setState(newValue);

      // Update search params without unnecessary cloning
      const newSearchParams = new URLSearchParams(searchParams);
      newSearchParams.set(key, String(newValue)); // Ensure value is set as string
      setSearchParams(newSearchParams);
    },
    [key, searchParams, setSearchParams]
  );

  return [state, setParamState];
}
export default useParamState;
