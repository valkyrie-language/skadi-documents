import type {AxiosInstance, AxiosRequestConfig, AxiosResponse} from 'axios';
import axios from 'axios';

// 创建axios实例
const request: AxiosInstance = axios.create({
    baseURL: 'http://localhost:8080',
    timeout: 5000,
    headers: {
        'Content-Type': 'application/json'
    }
});

// 请求拦截器
request.interceptors.request.use(
    (config: AxiosRequestConfig) => {
        // 在发送请求之前做些什么
        return config;
    },
    (error: any) => {
        // 对请求错误做些什么
        return Promise.reject(error);
    }
);

// 响应拦截器
request.interceptors.response.use(
    (response: AxiosResponse) => {
        // 对响应数据做点什么
        return response.data;
    },
    (error: any) => {
        // 对响应错误做点什么
        return Promise.reject(error);
    }
);

export default request;