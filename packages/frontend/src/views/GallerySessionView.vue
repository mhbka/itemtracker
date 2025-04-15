<template>
    <div class="session-view container mx-auto py-8 px-4">
      <div class="mb-4">
        <button @click="navigateBack" class="flex items-center text-blue-600 hover:text-blue-800">
          <span class="mr-2">←</span> Back to Gallery
        </button>
      </div>
  
      <div v-if="loading" class="flex justify-center my-8">
        <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"></div>
      </div>
      
      <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6">
        <p>{{ error }}</p>
      </div>
  
      <template v-else>
        <!-- Session Information -->
        <div class="bg-white shadow-md rounded-lg p-6 mb-8">
          <h1 class="text-2xl font-bold mb-4">Session #{{ session?.id }}</h1>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <div class="grid grid-cols-1 gap-2">
                <div class="flex">
                  <span class="font-medium w-32">Gallery ID:</span>
                  <span>{{ session?.gallery_id }}</span>
                </div>
                <div class="flex">
                  <span class="font-medium w-32">Created:</span>
                  <span>{{ formatUnixTimestamp(session?.created || 0) }}</span>
                </div>
                <div class="flex">
                  <span class="font-medium w-32">Total Items:</span>
                  <span>{{ session?.mercari_items.length || 0 }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
  
        <!-- Items List -->
        <div class="bg-white shadow-md rounded-lg p-6">
          <h2 class="text-xl font-bold mb-4">Marketplace Items</h2>
          
          <div v-if="session?.mercari_items.length === 0" class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded">
            <p>No items found in this session.</p>
          </div>
          
          <div v-else>
            <div v-for="(item, index) in session?.mercari_items" :key="item.item.item_id" 
                 class="mb-8 bg-gray-50 rounded-lg shadow p-4">
              <h3 class="text-lg font-semibold mb-3">{{ item.item.name }}</h3>
              
              <div class="flex flex-col md:flex-row gap-6">
                <!-- Item thumbnail -->
                <div class="w-full md:w-1/4 flex-shrink-0">
                  <img 
                    v-if="item.item.thumbnails?.length > 0" 
                    :src="item.item.thumbnails[0]" 
                    :alt="item.item.name"
                    class="w-full h-48 object-contain bg-white rounded border"
                  >
                  <div v-else class="w-full h-48 bg-gray-200 flex items-center justify-center rounded border">
                    <span class="text-gray-500">No image</span>
                  </div>
                </div>
                
                <!-- Item details -->
                <div class="w-full md:w-3/4">
                  <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-2">
                    <div class="flex">
                      <span class="font-medium w-32">ID:</span>
                      <span>{{ item.item.item_id }}</span>
                    </div>
                    <div class="flex">
                      <span class="font-medium w-32">Price:</span>
                      <span>{{ formatPrice(item.item.price) }}</span>
                    </div>
                    <div class="flex">
                      <span class="font-medium w-32">Status:</span>
                      <span>{{ item.item.status }}</span>
                    </div>
                    <div class="flex">
                      <span class="font-medium w-32">Seller ID:</span>
                      <span>{{ item.item.seller_id }}</span>
                    </div>
                    <div class="flex">
                      <span class="font-medium w-32">Category:</span>
                      <span>{{ item.item.category }}</span>
                    </div>
                    <div class="flex">
                      <span class="font-medium w-32">Condition:</span>
                      <span>{{ item.item.item_condition }}</span>
                    </div>
                    <div class="flex">
                      <span class="font-medium w-32">Created:</span>
                      <span>{{ formatUnixTimestamp(item.item.created) }}</span>
                    </div>
                    <div class="flex">
                      <span class="font-medium w-32">Updated:</span>
                      <span>{{ formatUnixTimestamp(item.item.updated) }}</span>
                    </div>
                  </div>
                  
                  <div class="mt-4">
                    <h4 class="font-medium mb-2">Description:</h4>
                    <p class="text-sm bg-white p-3 rounded border">{{ item.item.description }}</p>
                  </div>
                  
                  <div class="mt-4" v-if="item.evaluation_answers.length > 0">
                    <h4 class="font-medium mb-2">Evaluation Answers:</h4>
                    <div class="bg-white p-3 rounded border">
                      <div v-for="(answer, ansIndex) in item.evaluation_answers" :key="ansIndex" class="mb-2 last:mb-0">
                        <div class="flex flex-col text-sm">
                          <span class="font-medium">
                            {{ session?.used_evaluation_criteria.criteria[ansIndex]?.question || `Question ${ansIndex + 1}` }}:
                          </span>
                          <span>{{ formatCriterionAnswer(answer) }}</span>
                        </div>
                      </div>
                    </div>
                  </div>
                  
                  <div class="mt-4">
                    <h4 class="font-medium mb-2">Item Description:</h4>
                    <p class="text-sm bg-white p-3 rounded border">{{ item.item_description }}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </template>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted } from 'vue';
  import { useRoute, useRouter } from 'vue-router';
  import { fetchSession } from '@/services/api';
  import { formatUnixTimestamp, formatPrice, formatCriterionAnswer } from '@/utils/formatters';
  import type { GallerySession } from '@/types/galleries';
  
  const route = useRoute();
  const router = useRouter();
  const sessionId = ref(parseInt(route.params.id as string, 10));
  
  const session = ref<GallerySession | null>(null);
  const loading = ref(true);
  const error = ref<string | null>(null);
  
  onMounted(async () => {
    await fetchSessionData();
  });
  
  async function fetchSessionData() {
    loading.value = true;
    error.value = null;
    
    try {
      session.value = await fetchSession(sessionId.value);
    } catch (err) {
      error.value = 'Failed to load session data. Please try again later.';
      console.error(err);
    } finally {
      loading.value = false;
    }
  }
  
  function navigateBack() {
    if (session.value?.gallery_id) {
      router.push(`/gallery/${session.value.gallery_id}`);
    } else {
      router.push('/dashboard');
    }
  }
  </script>