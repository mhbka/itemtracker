<template>
    <div class="new-gallery-container">
      <h1 class="page-title">Create New Gallery</h1>
  
      <div v-if="error" class="error-message">
        <p>{{ error }}</p>
      </div>
  
      <div class="new-gallery-form">
        <form @submit.prevent="submitNewGallery">
          <div class="form-group">
            <label>Name</label>
            <input v-model="newGallery.name" type="text" required />
          </div>
  
          <div class="form-group">
            <label>Scraping Periodicity (as Cron string)</label>
            <input v-model="newGallery.scraping_periodicity" type="text" required />
          </div>
  
          <h3>Search Criteria</h3>
          <div class="form-group">
            <label>Keyword</label>
            <input v-model="newGallery.search_criteria.keyword" type="text" required />
          </div>
          <div class="form-group">
            <label>Exclude Keyword</label>
            <input v-model="newGallery.search_criteria.exclude_keyword" type="text" />
          </div>
          <div class="form-group">
            <label>Min Price</label>
            <input v-model.number="newGallery.search_criteria.min_price" type="number" />
          </div>
          <div class="form-group">
            <label>Max Price</label>
            <input v-model.number="newGallery.search_criteria.max_price" type="number" />
          </div>
  
          <h3>Evaluation Criteria</h3>
          <div
            class="form-group"
            v-for="(criterion, index) in newGallery.evaluation_criteria.criteria"
            :key="index"
          >
            <div class="form-group">
              <label>Question</label>
              <input v-model="criterion.question" type="text" required />
            </div>
            <div class="form-group">
              <label>Criterion Type</label>
              <input v-model="criterion.criterion_type" type="text" required />
            </div>
            <div class="form-group">
              <label>Hard Criterion (optional)</label>
              <input v-model="criterion.hard_criterion" type="text" />
            </div>
          </div>
          <button type="button" @click="addCriterion" class="primary-button">Add Criterion</button>
  
          <div class="form-actions">
            <button type="submit" class="primary-button">Create Gallery</button>
            <button type="button" @click="goBack" class="primary-button">Cancel</button>
          </div>
        </form>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import { useRouter } from 'vue-router';
  import { addNewGallery } from '@/services/api';
  import { Gallery, CriterionType, NewGallery } from '@/types/galleries';
  
  const router = useRouter();
  const error = ref<string | null>(null);
  const newGallery = ref<NewGallery>({
    name: '',
    is_active: true,
    scraping_periodicity: '0 0 * * *', // Default: daily at midnight
    search_criteria: {
      keyword: '',
      exclude_keyword: '',
      min_price: 0,
      max_price: 0
    },
    evaluation_criteria: {
      criteria: [
        {
          question: '',
          criterion_type: CriterionType.YesNo,
          hard_criterion: undefined
        }
      ]
    },
  });
  
  const submitNewGallery = async () => {
    try {
      await addNewGallery(newGallery.value);
      router.push('/dashboard'); // Navigate back to the dashboard after successful creation
    } catch (err) {
      error.value = 'Failed to create gallery. Please try again.';
      console.error(err);
    }
  };
  
  const addCriterion = () => {
    newGallery.value.evaluation_criteria.criteria.push({
      question: '',
      criterion_type: CriterionType.YesNo,
      hard_criterion: undefined,
    });
  };
  
  const goBack = () => {
    router.push('/dashboard');
  };
  </script>
  
  <style scoped>
  .new-gallery-container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem 1rem;
  }
  
  .page-title {
    font-size: 1.5rem;
    font-weight: bold;
    margin-bottom: 1.5rem;
  }
  
  .error-message {
    background-color: #fef2f2;
    border: 1px solid #fca5a5;
    color: #b91c1c;
    padding: 1rem;
    border-radius: 0.5rem;
    margin-bottom: 1rem;
  }
  
  .new-gallery-form {
    padding: 1.5rem;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.05);
  }
  
  .new-gallery-form h3 {
    margin: 1.5rem 0 1rem;
    font-size: 1.1rem;
  }
  
  .form-group {
    margin-bottom: 1rem;
    display: flex;
    flex-direction: column;
  }
  
  .form-group label {
    font-weight: 500;
    margin-bottom: 0.25rem;
  }
  
  .form-group input {
    padding: 0.5rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
  }
  
  .form-actions {
    margin-top: 2rem;
    display: flex;
    gap: 1rem;
  }
  
  .primary-button {
    padding: 0.5rem 1rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-weight: 500;
    cursor: pointer;
  }
  
  .primary-button:hover {
    background-color: #2563eb;
  }
  
  .secondary-button {
    padding: 0.5rem 1rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-weight: 500;
    cursor: pointer;
  }
  
  .secondary-button:hover {
    background-color: #e5e7eb;
  }
  </style>