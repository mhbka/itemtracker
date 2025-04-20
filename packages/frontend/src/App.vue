<template>
  <header @click="goHome" class="header-container">
    <img alt="logo" class="logo" src="@/assets/logo.svg" width="125" height="125" />
    <h1 class="main-title">itemtracker</h1>
    <p class="subtitle">track what you're looking for</p>

    <div v-if="!loading">
      <button v-if="!loggedIn" @click="signInWithGoogle" class="google-signin-btn">
        <img src="@/assets/google-icon.svg" class="button-icon" />
        Sign in with Google
      </button>
      <button v-else-if="loggedIn" @click="signOut">
        Sign out
      </button>
    </div>
  </header>

  <RouterView />
</template>

<script setup lang="ts">
import { RouterLink, RouterView, useRouter } from 'vue-router'
import { isLoggedIn } from './services/user'
import { supabase } from './main'
import { onMounted, ref } from 'vue'

const router = useRouter()

const loading = ref(true);
const loggedIn = ref(false);
const error = ref('');

onMounted(async () => {
  loggedIn.value = await isLoggedIn();
  loading.value = false;
})

async function goHome() {
  if (await isLoggedIn()) router.push('/dashboard');
  else router.push('/');
}

async function signInWithGoogle(): Promise<void> {
  try {
    const { error: signInError } = await supabase.auth.signInWithOAuth({
      provider: 'google',
      options: {
        redirectTo: `${window.location.origin}/dashboard`,
      },
    })

    if (signInError) {
      error.value = signInError.message
    }
  } catch (err) {
    error.value = 'An error occurred during sign in'
    console.error(err)
  }
}

async function signOut(): Promise<void> {
  await supabase.auth.signOut();
  router.push('/');
} 
</script>

<style>
#app {
  background-color: #1e1e1e;
  color: #dbdbdb;
  font-family: 'Courier New', Courier, monospace;
  min-height: 100vh; /* Ensure it covers the full viewport height */
}

.logo {
  filter: invert(11%) sepia(77%) saturate(5212%) hue-rotate(244deg) brightness(90%) contrast(100%);
}

.button-icon {
  height: 1.5em;
  vertical-align: middle;
}

.header-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 1rem 0;
  cursor: pointer;
}

.main-title {
  color: #4141ff;
  font-size: 3rem;
  margin-bottom: 0.5rem;
  margin-top: 0.5rem;
}

.subtitle {
  font-size: 1.2rem;
  margin-top: 0;
  opacity: 0.8;
}

button,
input,
select,
textarea {
  font-family: 'Courier New', Courier, monospace;
  background-color: #414141;
  color: #dbdbdb;
  padding: 0.5rem 1rem;
  border: 1px solid #d1d5db;
  border-radius: 0.375rem;
  font-weight: 500;
}

button {
  cursor: pointer;
}

button:hover {
  background-color: #1e40af;
}

tr {
  border-bottom: 1px solid #e5e7eb;
  cursor: pointer;
}

th {
  padding: 0.75rem 1.5rem;
  text-align: left;
}

tr:hover {
  background-color: #3d3d3d;
}

tr td {
  padding: 0.75rem 1.5rem;
  text-align: left;
}

body,
html {
  margin: 0;
  padding: 0;
  align-items: center;
}
</style>
