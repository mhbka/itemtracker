<template>
  <div class="home-container">
    <h1>Welcome to Our App</h1>
    <p>Sign in to access your personalized dashboard and more.</p>
    <div class="actions">
      <router-link to="/login" class="login-link">Sign In</router-link>
    </div>
  </div>
</template>

<style scoped>
.home-container {
  max-width: 800px;
  margin: 100px auto;
  text-align: center;
  padding: 20px;
}

.actions {
  margin-top: 30px;
}

.login-link {
  display: inline-block;
  background-color: #4285f4;
  color: white;
  padding: 12px 24px;
  border-radius: 4px;
  text-decoration: none;
  font-weight: bold;
}
</style>
