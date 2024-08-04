import axios, { AxiosResponse } from "axios";

axios.defaults.baseURL = "http://chinslt.com:3011/";
console.log("axios base url: " + axios.defaults.baseURL);

const responseBody = <T>(response: AxiosResponse<T>) => response.data;

const requests = {
  get: <T>(url: string) => axios.get<T>(url).then(responseBody),
  getParams: <T>(url: string, paramsBody: {}) =>
    axios.get<T>(url, { params: paramsBody }).then(responseBody),

  post: <T>(url: string, body: {}) =>
    axios.post<T>(url, body).then(responseBody),
  put: <T>(url: string, body: {}) => axios.put<T>(url, body).then(responseBody),
  del: <T>(url: string) => axios.delete<T>(url).then(responseBody),
};

export default requests;