<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <header>
    <img alt="Vue logo" class="logo" src="@/assets/logo.svg" width="125" height="125" />
  </header>

  <RouterView />
</template>

<style>
#app {
  background-color: #1e1e1e;
  color: #e8e8e8cf;
  font-family: 'Courier New', Courier, monospace;
  min-height: 100vh; /* Ensure it covers the full viewport height */
}

button, input, select, textarea {
  font-family: 'Courier New', Courier, monospace;
  background-color: #414141; 
  color: #e8e8e8cf;
  padding: 0.5rem 1rem;
  border: 1px solid #d1d5db;
  border-radius: 0.375rem;
  font-weight: 500;
}

button {
  cursor: pointer
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

body, html {
  margin: 0;
  padding: 0;
  align-items: center;
}
</style>
