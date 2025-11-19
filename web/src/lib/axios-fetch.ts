// Adopted from https://juejin.cn/post/7237840998985072698

import axios from 'axios';
import { toast } from 'sonner';

import { recursiveDateConversion } from './date-utils';

import type {
  AxiosInstance,
  AxiosResponse,
  CreateAxiosDefaults,
  InternalAxiosRequestConfig,
} from 'axios';
import { useCommonStore } from '@/common/store';
import { kspaceStore } from '@/krate/kspace/store';

class Request {
  private instance: AxiosInstance;
  private abortControllerMap: Map<string, AbortController>;

  constructor(config: CreateAxiosDefaults) {
    this.instance = axios.create(config);

    this.abortControllerMap = new Map();

    this.instance.interceptors.request.use((config: InternalAxiosRequestConfig) => {
      useCommonStore.getState().appendLog(config.baseURL ?? '');
      const kspace = kspaceStore.getState();
      config.headers!['K-kspace'] = kspace.currentKSpace;
      config.headers!['K-mkspaces'] = kspace.mkspaces.join(',');

      const controller = new AbortController();
      const url = config.url || '';
      config.signal = controller.signal;

      this.abortControllerMap.set(url, controller);

      return config;
    }, Promise.reject);
    this.instance.interceptors.response.use(
      (response: AxiosResponse) => {
        useCommonStore.getState().appendLog(response.data ?? '');
        const url = response.config.url || '';
        this.abortControllerMap.delete(url);

        const data = recursiveDateConversion(response.data);
        response.data = data;
        return response;
      },
      (err: Error) => {
        useCommonStore.getState().appendLog(`${err.message} -- ${err.stack} -- ${err.name}`);

        toast.error(`Error! ${err.name}, ${err.message} ${err.stack} ${err.cause}`);

        return Promise.reject(err);
      },
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

  async postFormdata<T>(url: string, data: FormData): Promise<T> {
    return (await this.instance.post<T>(url, data)).data;
  }

  async postJson<T, E>(url: string, data?: E): Promise<T> {
    return (await this.instance.post<T>(url, data)).data;
  }

  async putJson<T, E>(url: string, data?: E): Promise<T> {
    return (await this.instance.put<T>(url, data)).data;
  }
}

export const BASE_URL = import.meta.env.DEV
  ? import.meta.env.PUBLIC_BACKEND_URL
  : window.location.protocol + '//' + window.location.host;

const request = new Request({
  timeout: 30 * 1000,
  baseURL: BASE_URL,
});

export default request;
