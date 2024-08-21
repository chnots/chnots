// Adopted from https://juejin.cn/post/7237840998985072698

import { useDomainStore } from "@/store/v1/domain";
import axios from "axios";
import type {
  AxiosInstance,
  AxiosRequestConfig,
  AxiosResponse,
  CreateAxiosDefaults,
  InternalAxiosRequestConfig,
} from "axios";
import { toast } from "sonner";
import { recursiveDateConversion } from "./axios-date-transformer";

const responseBody = <T>(response: AxiosResponse<T>) => response.data;

class Request {
  private instance: AxiosInstance;
  private abortControllerMap: Map<string, AbortController>;

  constructor(config: CreateAxiosDefaults) {
    this.instance = axios.create(config);

    this.abortControllerMap = new Map();

    this.instance.interceptors.request.use(
      (config: InternalAxiosRequestConfig) => {
        if (config.url !== "/login") {
          const domain = useDomainStore.getState().current.name;
          if (domain) {
            config.headers!["K-Domain"] = domain;
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
    const response = await this.instance.get<T>(url, config);
    return responseBody(response);
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
  baseURL: window.location.protocol + "//" + "chinslt.com:3012",
});

export default request;
