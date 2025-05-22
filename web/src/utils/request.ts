// Adopted from https://juejin.cn/post/7237840998985072698

import { useKSpaceStore } from "@/store/kspace";
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
        const kspace = useKSpaceStore.getState().currentKSpace.name;
        config.headers!["K-kspace"] = kspace;

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

        const data = recursiveDateConversion(response.data);
        response.data = data;
        return response;
      },
      (err: Error) => {
        toast.error("Axios Error Occured! " + err);

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

  async get<T, E>(url: string, params?: E): Promise<T> {
    return (
      await this.instance.get<T>(url, {
        params,
      })
    ).data;
  }

  async post<T, E>(url: string, data?: E): Promise<T> {
    return (await this.instance.post<T>(url, data)).data;
  }

  async put<T, E>(
    url: string,
    data?: E,
    headers?: Record<string, string>
  ): Promise<T> {
    return (await this.instance.put<T>(url, data)).data;
  }
}

export const BASE_URL = import.meta.env.DEV
  ? import.meta.env.PUBLIC_BACKEND_URL
  : window.location.protocol + "//" + window.location.host;

const request = new Request({
  timeout: 30 * 1000,
  baseURL: BASE_URL,
});

export default request;
