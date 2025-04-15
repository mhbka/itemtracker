// src/main.ts
import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createClient } from '@supabase/supabase-js'

const app = createApp(App)

// Supabase client initialization
const supabaseUrl = import.meta.env.VITE_SUPABASE_URL as string
const supabaseAnonKey = import.meta.env.VITE_SUPABASE_ANON_KEY as string

export const supabase = createClient(supabaseUrl, supabaseAnonKey)

app.provide('supabase', supabase)
app.use(router)
app.mount('#app')