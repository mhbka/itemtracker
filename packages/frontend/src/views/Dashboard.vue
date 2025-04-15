<template>
  <div class="posts-page">
    <PostForm @post-added="handlePostAdded" />
    <PostsList 
      :posts="posts" 
      :loading="loading" 
      :error="error" 
    />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import PostsList from '../components/PostsList.vue'
import PostForm from '../components/PostForm.vue'
import { fetchUserPosts } from '@/services/user'

const posts = ref([])
const loading = ref(false)
const error = ref('')

const fetchPosts = async () => {
  loading.value = true
  error.value = ''
  
  try {
    posts.value = await fetchUserPosts();
  } catch (err) {
    error.value = err.message || 'An error occurred while fetching posts'
  } finally {
    loading.value = false
  }
}

const handlePostAdded = async () => {
  await fetchPosts();
}

onMounted(() => {
  fetchPosts()
})
</script>

<style scoped>
.posts-page {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}
</style>