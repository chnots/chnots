import { useCommonStore } from "@/common/store";
import { recursiveDateConversion } from "./date-utils";
import { useKSpaceStore } from "@/krate/kspace/store";
import { toast } from "sonner";

const appendUrl = (base: string, suffix: string) => {
  if (base.endsWith("/") && suffix.startsWith("/")) {
    return base + suffix.substring(1);
  } else if (base.endsWith("/") || suffix.startsWith("/")) {
    return base + suffix;
  } else {
    return base + "/" + suffix;
  }
};

class FetchRequest {
  private abortControllerMap: Map<string, AbortController>;
  private baseConfig: RequestInit;
  private baseURL: string;

  constructor(config: { baseURL: string; timeout?: number }) {
    this.abortControllerMap = new Map();
    this.baseURL = config.baseURL;
    this.baseConfig = {
      headers: {
        "Content-Type": "application/json",
      },
      // Convert timeout to AbortSignal
      signal: AbortSignal.timeout(config.timeout || 30 * 1000),
    };
  }

  private async requestInterceptor(
    config: RequestInit & { url: string }
  ): Promise<RequestInit> {
    useCommonStore.getState().appendLog(this.baseURL);
    const kspace = useKSpaceStore.getState();

    const controller = new AbortController();
    this.abortControllerMap.set(config.url, controller);

    return {
      ...this.baseConfig,
      ...config,
      headers: {
        ...this.baseConfig.headers,
        ...config.headers,
        "K-kspace": kspace.currentKSpace,
        "K-mkspaces": kspace.mkspaces.join(","),
      },
      signal: controller.signal,
    };
  }

  private async responseInterceptor<T>(response: Response): Promise<T> {
    const url = response.url.split(this.baseURL)[1] || "";
    this.abortControllerMap.delete(url);

    if (!response.ok) {
      const error = new Error(`HTTP error! status: ${response.status}`);
      useCommonStore.getState().appendLog(`${error.message}`);
      throw error;
    }

    const data = await response.json();
    useCommonStore.getState().appendLog(JSON.stringify(data));
    return recursiveDateConversion(data);
  }

  cancelAllRequest() {
    for (const [, controller] of this.abortControllerMap) {
      controller.abort();
    }
    this.abortControllerMap.clear();
  }

  cancelRequest(url: string | string[]) {
    const urlList = Array.isArray(url) ? url : [url];
    for (const _url of urlList) {
      this.abortControllerMap.get(_url)?.abort();
      this.abortControllerMap.delete(_url);
    }
  }

  async get<T, E>(url: string, params?: E): Promise<T> {
    const query = params
      ? `?${new URLSearchParams(params as Record<string, string>)}`
      : "";
    const fullUrl = appendUrl(this.baseURL, `${url}${query}`);
    console.log(fullUrl);
    const config = await this.requestInterceptor({ url, method: "GET" });

    const response = await fetch(fullUrl, config);
    return this.responseInterceptor<T>(response);
  }

  async post<T, E>(url: string, data?: E): Promise<T> {
    const fullUrl = appendUrl(this.baseURL, url);
    const config = await this.requestInterceptor({
      url,
      method: "POST",
      body: JSON.stringify(data),
    });

    const response = await fetch(fullUrl, config);
    return this.responseInterceptor<T>(response);
  }

  async put<T, E>(
    url: string,
    data?: E,
    headers?: Record<string, string>
  ): Promise<T> {
    const fullUrl = appendUrl(this.baseURL, url);
    const config = await this.requestInterceptor({
      url,
      method: "PUT",
      body: JSON.stringify(data),
      headers: headers ? { ...headers } : undefined,
    });

    const response = await fetch(fullUrl, config);
    return this.responseInterceptor<T>(response);
  }
}

export const BASE_URL = "http://127.0.0.1:3012";

const request = new FetchRequest({
  baseURL: BASE_URL,
  timeout: 30 * 1000,
});

export default request;
