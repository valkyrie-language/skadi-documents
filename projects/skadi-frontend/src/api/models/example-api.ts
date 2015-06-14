import request from '../request';

// 定义接口返回数据类型
interface ExampleResponse {
  data: any;
  message: string;
  code: number;
}

// 示例GET请求
export const getExampleData = () => {
  return request.get<ExampleResponse>('/example');
};

// 示例POST请求
export const postExampleData = (data: any) => {
  return request.post<ExampleResponse>('/example', data);
};

// 示例PUT请求
export const updateExampleData = (id: string, data: any) => {
  return request.put<ExampleResponse>(`/example/${id}`, data);
};

// 示例DELETE请求
export const deleteExampleData = (id: string) => {
  return request.delete<ExampleResponse>(`/example/${id}`);
};