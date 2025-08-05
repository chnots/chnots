import axiosRequest, { BASE_URL as axiosBaseUrl } from "./axios-fetch";
import tauriRequest, { BASE_URL as tauriBaseUrl } from "./tauri-fetch";

const hasTauriProperty = (): boolean => {
  if (typeof window === "undefined") {
    return false;
  }

  const propertyNames = Object.getOwnPropertyNames(window);
  console.log("all properties,", propertyNames);

  return propertyNames.some((prop) => prop.startsWith("__TAURI_"));
};

// Determine environment
const isTauri = hasTauriProperty();

console.log(
  "isTauri",
  isTauri,
  typeof window !== "undefined",
  hasTauriProperty(),
);

// Select the appropriate implementation
const request = isTauri ? tauriRequest : axiosRequest;
const BASE_URL = isTauri ? tauriBaseUrl : axiosBaseUrl;

export default request;
export { BASE_URL, isTauri };
