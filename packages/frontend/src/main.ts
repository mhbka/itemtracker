import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createClient } from '@supabase/supabase-js'

const app = createApp(App)

const supabaseUrl = import.meta.env.VITE_SUPABASE_URL
const supabaseAnonKey = import.meta.env.VITE_SUPABASE_ANON_KEY

export const supabase = createClient(supabaseUrl, supabaseAnonKey)

app.provide('supabase', supabase)
app.use(router)
app.mount('#app')
