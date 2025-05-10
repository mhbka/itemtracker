import { supabase } from '../main'
import axios from 'axios'

let API_URL: string;
if (import.meta.env.DEV) 
  API_URL = `http://${import.meta.env.VITE_API_URL}`;
else 
  // TODO: change this to https when DNS is properly configured for it
  API_URL = `http://${import.meta.env.VITE_API_URL}`;

const apiClient = axios.create({
  baseURL: API_URL,
  timeout: 10000,
})

apiClient.interceptors.request.use(
  async (config) => {
    const session = await supabase.auth.getSession()
    const token = session.data.session.access_token
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error) => {
    return Promise.reject(error)
  },
)

export default apiClient
