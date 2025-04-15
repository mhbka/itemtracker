<template>
    <div class="login-container">
      <h1>Sign in to your account</h1>
      <div v-if="error" class="error-message">{{ error }}</div>
      <button @click="signInWithGoogle" class="google-signin-btn">
        <img src="../assets/google-icon.svg" alt="Google" />
        Sign in with Google
      </button>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, inject } from 'vue'
  import { useRouter } from 'vue-router'
  import { SupabaseClient } from '@supabase/supabase-js'
  
  const supabase = inject('supabase') as SupabaseClient
  const router = useRouter()
  const error = ref<string>('')
  
  const signInWithGoogle = async (): Promise<void> => {
    try {
      const { error: signInError } = await supabase.auth.signInWithOAuth({
        provider: 'google',
        options: {
          redirectTo: `${window.location.origin}/dashboard`
        }
      })
      
      if (signInError) {
        error.value = signInError.message
      }
    } catch (err) {
      error.value = 'An error occurred during sign in'
      console.error(err)
    }
  }
  </script>
  
  <style scoped>
  .login-container {
    max-width: 400px;
    margin: 100px auto;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    text-align: center;
  }
  
  .google-signin-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 10px 20px;
    border: 1px solid #ddd;
    border-radius: 4px;
    background-color: white;
    cursor: pointer;
    font-size: 16px;
    margin: 20px auto;
    width: 100%;
    max-width: 300px;
  }
  
  .google-signin-btn img {
    margin-right: 10px;
    width: 20px;
    height: 20px;
  }
  
  .error-message {
    color: red;
    margin-bottom: 15px;
  }
  
  h1 {
    margin-bottom: 30px;
  }
  </style>