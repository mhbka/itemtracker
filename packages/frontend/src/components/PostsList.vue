<template>
    <div class="posts-container">
      <h1>Posts</h1>
      
      <!-- Loading state -->
      <div v-if="loading" class="loading">
        <p>Loading posts...</p>
      </div>
      
      <!-- Error state -->
      <div v-else-if="error" class="error">
        <p>Error: {{ error }}</p>
      </div>
      
      <!-- Empty state -->
      <div v-else-if="posts.length === 0" class="empty">
        <p>No posts available.</p>
      </div>
      
      <!-- Posts list -->
      <ul v-else class="posts-list">
        <li v-for="post in posts" :key="post.id" class="post-item">
          <h2>{{ post.title }}</h2>
          <p>{{ post.body }}</p>
          <div class="post-meta">
            <span v-if="post.author">Author: {{ post.author }}</span>
            <span v-if="post.createdAt">Published: {{ formatDate(post.createdAt) }}</span>
          </div>
        </li>
      </ul>
    </div>
  </template>
  
  <script setup>
  import { defineProps } from 'vue'
  
  const props = defineProps({
    posts: {
      type: Array,
      default: () => []
    },
    loading: {
      type: Boolean,
      default: false
    },
    error: {
      type: String,
      default: ''
    }
  })
  
  const formatDate = (dateString) => {
    const date = new Date(dateString)
    return date.toLocaleDateString()
  }
  </script>
  
  <style scoped>
  .posts-container {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
  }
  
  .posts-list {
    list-style: none;
    padding: 0;
  }
  
  .post-item {
    border-bottom: 1px solid #eee;
    margin-bottom: 20px;
    padding-bottom: 20px;
  }
  
  .post-item:last-child {
    border-bottom: none;
  }
  
  .post-item h2 {
    margin-top: 0;
    font-size: 1.4em;
  }
  
  .post-meta {
    font-size: 0.85em;
    color: #666;
    display: flex;
    gap: 15px;
  }
  
  .loading, .error, .empty {
    text-align: center;
    padding: 30px;
    background: #f8f8f8;
    border-radius: 4px;
  }
  
  .error {
    color: #d32f2f;
  }
  </style>