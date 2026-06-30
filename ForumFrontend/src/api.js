// ==========================================
// 阶段一：模块与依赖引入
// ==========================================
import axios from 'axios';

// ==========================================
// 阶段二：Axios 实例配置与初始化
// ==========================================
const api = axios.create({
  baseURL: 'http://127.0.0.1:30002',
  timeout: 10000,
  withCredentials: true,
  headers: {
    'Content-Type': 'application/json'
  },
  transformResponse: [function (data) {
    if (typeof data === 'string') {
      try {
        // Fix JavaScript int64 precision loss by wrapping large IDs in quotes before parsing
        const fixedData = data.replace(/"(user_id|author_id|article_id|id)":\s*(\d{15,20})/g, '"$1":"$2"');
        return JSON.parse(fixedData);
      } catch (e) {
        return data;
      }
    }
    return data;
  }]
});

// ==========================================
// 阶段三：请求/响应拦截器逻辑
// ==========================================
let isRefreshing = false;
let failedQueue = [];

const processQueue = (error, token = null) => {
  failedQueue.forEach(prom => {
    if (error) {
      prom.reject(error);
    } else {
      prom.resolve(token);
    }
  });
  failedQueue = [];
};

// Request interceptor to add token
api.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('access_token');
    if (token) {
      config.headers['Authorization'] = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

api.interceptors.response.use(
  (response) => {
    return response;
  },
  async (error) => {
    const originalRequest = error.config;
    
    // Check if error is 401 and we haven't retried yet
    if (error.response && error.response.status === 401 && !originalRequest._retry) {
      if (isRefreshing) {
        return new Promise(function(resolve, reject) {
          failedQueue.push({ resolve, reject });
        }).then(token => {
          originalRequest.headers['Authorization'] = 'Bearer ' + token;
          return api(originalRequest);
        }).catch(err => {
          return Promise.reject(err);
        });
      }

      originalRequest._retry = true;
      isRefreshing = true;

      try {
        // Assume backend refresh token endpoint is /user/refresh_token
        // We use a fresh axios instance to avoid interceptor loops
        const refreshResponse = await axios.post('http://127.0.0.1:30002/user/refresh_token', {}, {
          withCredentials: true
        });
        
        let data = refreshResponse.data;
        if (data.errCode === 0 && data.data) {
          data = data.data;
        }

        if (data.accessToken) {
          localStorage.setItem('access_token', data.accessToken);
          api.defaults.headers.common['Authorization'] = 'Bearer ' + data.accessToken;
          processQueue(null, data.accessToken);
          
          originalRequest.headers['Authorization'] = 'Bearer ' + data.accessToken;
          return api(originalRequest);
        } else {
          throw new Error('No token returned');
        }
      } catch (refreshError) {
        processQueue(refreshError, null);
        // Clean up and notify
        localStorage.removeItem('access_token');
        localStorage.removeItem('refresh_token');
        localStorage.removeItem('user_email');
        localStorage.removeItem('user_nickname');
        localStorage.removeItem('user_id');
        
        window.dispatchEvent(new Event('auth-expired'));
        return Promise.reject(refreshError);
      } finally {
        isRefreshing = false;
      }
    }
    
    return Promise.reject(error);
  }
);

export default api;
