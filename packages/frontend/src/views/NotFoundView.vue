<template>
  <div class="flex flex-col items-center justify-center h-screen">
    <h1 class="text-6xl font-bold text-gray-800 mb-4">404</h1>
    <p class="text-xl text-gray-600 mb-8">Page not found</p>
    <router-link
      to="/dashboard"
      class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
    >
      Back to Dashboard
    </router-link>
  </div>
</template>
