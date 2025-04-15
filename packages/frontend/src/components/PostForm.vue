<template>
    <div class="add-post-form">
      <h2>Add New Post</h2>
      
      <form @submit.prevent="submitPost">
        <div class="form-group">
          <label for="post-title">Title</label>
          <input
            id="post-title"
            v-model="newPost.title"
            type="text"
            placeholder="Post title"
            required
          />
        </div>
        
        <div class="form-group">
          <label for="post-body">Content</label>
          <textarea
            id="post-body"
            v-model="newPost.body"
            placeholder="Write your post content here..."
            rows="5"
            required
          ></textarea>
        </div>
        
        <div class="form-controls">
          <button type="button" class="btn-cancel" @click="resetForm">Clear</button>
          <button type="submit" class="btn-submit" :disabled="isSubmitting">
            {{ isSubmitting ? 'Submitting...' : 'Add Post' }}
          </button>
        </div>
        
        <div v-if="error" class="form-error">
          {{ error }}
        </div>
      </form>
    </div>
  </template>
  
  <script setup>
import { addUserPost } from '@/services/user'
import { ref, reactive } from 'vue'
import { defineEmits } from 'vue'

const emit = defineEmits(['post-added'])

const newPost = reactive({
  title: '',
  body: ''
})

const isSubmitting = ref(false)
const error = ref('')

const resetForm = () => {
  newPost.title = ''
  newPost.body = ''
  error.value = ''
}

const submitPost = async () => {
  try {
    isSubmitting.value = true;
    error.value = '';

    const response = await addUserPost(newPost.title, newPost.body);
    emit('post-added');
    
    resetForm();
  } catch (err) {
    error.value = err.message || 'An error occurred while adding the post'
  } finally {
    isSubmitting.value = false
  }
}
</script>
  
  <style scoped>
  .add-post-form {
    background: #f9f9f9;
    padding: 20px;
    border-radius: 8px;
    margin-bottom: 30px;
  }
  
  .form-group {
    margin-bottom: 15px;
  }
  
  .form-group label {
    display: block;
    margin-bottom: 5px;
    font-weight: bold;
  }
  
  input[type="text"],
  textarea {
    width: 100%;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-family: inherit;
    font-size: 1em;
  }
  
  .form-controls {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 15px;
  }
  
  button {
    padding: 10px 15px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: bold;
    transition: background-color 0.2s;
  }
  
  .btn-submit {
    background-color: #4caf50;
    color: white;
  }
  
  .btn-submit:hover {
    background-color: #45a049;
  }
  
  .btn-submit:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
  }
  
  .btn-cancel {
    background-color: #f1f1f1;
    color: #333;
  }
  
  .btn-cancel:hover {
    background-color: #e5e5e5;
  }
  
  .form-error {
    color: #d32f2f;
    margin-top: 15px;
    padding: 10px;
    background-color: #ffebee;
    border-radius: 4px;
  }
  </style>