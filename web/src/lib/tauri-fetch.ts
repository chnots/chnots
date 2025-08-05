import { recursiveDateConversion } from "./date-utils";
import { kspaceStore } from "@/krate/kspace/store";

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
  private baseConfig: RequestInit;
  private baseURL: string;

  constructor(config: { baseURL: string; timeout?: number }) {
    this.baseURL = config.baseURL;
    this.baseConfig = {
      signal: AbortSignal.timeout(config.timeout || 30 * 1000),
    };
  }

  private async requestInterceptor(
    config: RequestInit & { url: string },
  ): Promise<RequestInit> {
    const kspace = kspaceStore.getState();

    const controller = new AbortController();

    const req = {
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
    console.log("request, ", JSON.stringify(req));
    return req;
  }

  private async responseInterceptor<T>(response: Response): Promise<T> {
    if (!response.ok) {
      const error = new Error(`HTTP error! status: ${response.status}`);
      throw error;
    }

    const data = await response.json();
    return recursiveDateConversion(data);
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

  async postFormdata<T>(url: string, data: FormData): Promise<T> {
    const fullUrl = appendUrl(this.baseURL, url);
    let body: BodyInit = data;
    const config = await this.requestInterceptor({
      url,
      method: "POST",
      body,
    });

    const response = await fetch(fullUrl, config);
    return this.responseInterceptor<T>(response);
  }

  async postJson<T, E>(url: string, data?: E): Promise<T> {
    const fullUrl = appendUrl(this.baseURL, url);
    const config = await this.requestInterceptor({
      url,
      headers: {
        "Content-Type": "application/json",
      },
      method: "POST",
      body: JSON.stringify(data),
    });

    const response = await fetch(fullUrl, config);
    return this.responseInterceptor<T>(response);
  }

  async putJson<T, E>(url: string, data?: E): Promise<T> {
    const fullUrl = appendUrl(this.baseURL, url);
    const config = await this.requestInterceptor({
      url,
      method: "PUT",
      body: JSON.stringify(data),
      headers: {
        "Content-Type": "application/json",
      },
    });

    const response = await fetch(fullUrl, config);
    return this.responseInterceptor<T>(response);
  }
}

export const BASE_URL = "http://127.0.0.1:3013";

const request = new FetchRequest({
  baseURL: BASE_URL,
  timeout: 30 * 1000,
});

export default request;
