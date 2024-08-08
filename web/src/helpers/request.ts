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
// import { useUserInfoStore } from "~/stores";

const responseBody = <T>(response: AxiosResponse<T>) => response.data;

class Request {
  private instance: AxiosInstance;
  // 存放取消请求控制器Map
  private abortControllerMap: Map<string, AbortController>;

  constructor(config: CreateAxiosDefaults) {
    this.instance = axios.create(config);

    this.abortControllerMap = new Map();

    // 请求拦截器
    this.instance.interceptors.request.use(
      (config: InternalAxiosRequestConfig) => {
        if (config.url !== "/login") {
          const domain = useDomainStore.getState().current.name;
          if (domain) config.headers!["K-Domain"] = domain;
        }

        const controller = new AbortController();
        const url = config.url || "";
        config.signal = controller.signal;
        this.abortControllerMap.set(url, controller);

        return config;
      },
      Promise.reject
    );

    // 响应拦截器
    this.instance.interceptors.response.use(
      (response: AxiosResponse) => {
        const url = response.config.url || "";
        this.abortControllerMap.delete(url);

        console.log("processing response.");

        return response;
      },
      (err) => {
        /*        if (err.response?.status === 401) {
          // 登录态失效，清空userInfo，跳转登录页
          useUserInfoStore.setState({ userInfo: null });
          window.location.href = `/login?redirect=${window.location.pathname}`;
        } */

        console.log("unable to read response: ", err);

        return Promise.reject(err);
      }
    );
  }

  // 取消全部请求
  cancelAllRequest() {
    for (const [, controller] of this.abortControllerMap) {
      controller.abort();
    }
    this.abortControllerMap.clear();
  }

  // 取消指定的请求
  cancelRequest(url: string | string[]) {
    const urlList = Array.isArray(url) ? url : [url];
    for (const _url of urlList) {
      this.abortControllerMap.get(_url)?.abort();
      this.abortControllerMap.delete(_url);
    }
  }

  async request<T>(config: AxiosRequestConfig): Promise<T> {
    return this.instance.request<T>(config).then(responseBody);
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
    const response = await this.instance.get<T>(url, {
      ...config,
      params: data,
    });
    return responseBody(response);
  }

  async post<T>(
    url: string,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    data?: any,
    config?: AxiosRequestConfig
  ): Promise<T> {
    const response = await this.instance.post<T>(url, data, config);
    return responseBody(response);
  }
}

const request = new Request({
  timeout: 20 * 1000,
  baseURL: "http://chinslt.com:3011",
});

export default request;
