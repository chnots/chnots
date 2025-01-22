// Adopted from https://juejin.cn/post/7237840998985072698

import { useNamespaceStore } from "@/store/namespace";
import axios from "axios";
import type {
  AxiosInstance,
  AxiosRequestConfig,
  AxiosResponse,
  CreateAxiosDefaults,
  InternalAxiosRequestConfig,
} from "axios";
import { toast } from "sonner";
import { recursiveDateConversion } from "./date-utils";

class Request {
  private instance: AxiosInstance;
  private abortControllerMap: Map<string, AbortController>;

  constructor(config: CreateAxiosDefaults) {
    this.instance = axios.create(config);

    this.abortControllerMap = new Map();

    this.instance.interceptors.request.use(
      (config: InternalAxiosRequestConfig) => {
        if (config.url !== "/login") {
          const namespace = useNamespaceStore.getState().currentNamespace.name;
          if (namespace) {
            config.headers!["K-namespace"] = namespace;
          }
        }

        const controller = new AbortController();
        const url = config.url || "";
        config.signal = controller.signal;
        this.abortControllerMap.set(url, controller);

        return config;
      },
      Promise.reject
    );

    this.instance.interceptors.response.use(
      (response: AxiosResponse) => {
        const url = response.config.url || "";
        this.abortControllerMap.delete(url);

        return recursiveDateConversion(response.data);
      },
      (err) => {
        /*        if (err.response?.status === 401) {
          // 登录态失效，清空userInfo，跳转登录页
          useUserInfoStore.setState({ userInfo: null });
          window.location.href = `/login?redirect=${window.location.pathname}`;
        } */

        toast.error("Fail " + err);

        return Promise.reject(err);
      }
    );
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

  async get<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    // @ts-ignore
    return await this.instance.get<T>(url, config);
  }

  async query<T>(
    url: string,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    data?: any,
    config?: AxiosRequestConfig
  ): Promise<T> {
    // @ts-ignore
    return await this.instance.get<T>(url, {
      ...config,
      params: data,
    });
  }

  async post<T>(
    url: string,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    data?: any,
    config?: AxiosRequestConfig
  ): Promise<T> {
    // @ts-ignore
    return await this.instance.post<T>(url, data, config);
  }

  async put<T>(
    url: string,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    data?: any,
    config?: AxiosRequestConfig
  ): Promise<T> {
    // @ts-ignore
    return await this.instance.put<T>(url, data, config);
  }
}

const request = new Request({
  timeout: 20 * 1000,
  baseURL: import.meta.env.DEV
    ? import.meta.env.PUBLIC_BACKEND_URL
    : window.location.protocol + "//" + window.location.host,
});

export default request;
