<template>
  <header @click="goHome" class="header-container">
    <!--
    <img alt="Vue logo" class="logo" src="@/assets/logo.svg" width="125" height="125" />
    -->

    <h1 class="main-title">itemtracker</h1>
    <p class="subtitle">track what you're looking for</p>
  </header>

  <RouterView />
</template>

<script setup lang="ts">
import { RouterLink, RouterView, useRouter } from 'vue-router'
import { isLoggedIn } from './services/user'

const router = useRouter()

async function goHome() {
  if (await isLoggedIn()) router.push('/dashboard')
  else router.push('/')
}
</script>

<style>
#app {
  background-color: #1e1e1e;
  color: #dbdbdb;
  font-family: 'Courier New', Courier, monospace;
  min-height: 100vh; /* Ensure it covers the full viewport height */
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
  color: rgb(65, 65, 255);
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
