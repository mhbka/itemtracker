import { supabase } from '../main';
import axios from 'axios';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

const apiClient = axios.create({
  baseURL: API_URL,
  timeout: 10000
});

apiClient.interceptors.request.use(
  async config => {
    const session = await supabase.auth.getSession();
    const token = session.data.session.access_token;
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  error => {
    return Promise.reject(error);
  }
);

export default apiClient;