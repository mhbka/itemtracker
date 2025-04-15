<template>
    <div class="gallery-view container mx-auto py-8 px-4">
      <div class="mb-4">
        <button @click="$router.push('/dashboard')" class="flex items-center text-blue-600 hover:text-blue-800">
          <span class="mr-2">←</span> Back to Dashboard
        </button>
      </div>
  
      <div v-if="loading" class="flex justify-center my-8">
        <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"></div>
      </div>
      
      <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-6">
        <p>{{ error }}</p>
      </div>
  
      <template v-else>
        <!-- Gallery Details -->
        <div class="bg-white shadow-md rounded-lg p-6 mb-8">
          <h1 class="text-2xl font-bold mb-4">{{ gallery?.name || 'Gallery Details' }}</h1>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h3 class="text-lg font-semibold mb-2">Basic Information</h3>
              <div class="grid grid-cols-1 gap-2">
                <div class="flex">
                  <span class="font-medium w-32">ID:</span>
                  <span>{{ gallery?.id }}</span>
                </div>
                <div class="flex">
                  <span class="font-medium w-32">Status:</span>
                  <span :class="gallery?.is_active ? 'text-green-600' : 'text-red-600'">
                    {{ gallery?.is_active ? 'Active' : 'Inactive' }}
                  </span>
                </div>
                <div class="flex">
                  <span class="font-medium w-32">Schedule:</span>
                  <span>{{ gallery?.scraping_periodicity }}</span>
                </div>
                <div class="flex">
                  <span class="font-medium w-32">Last Scraped:</span>
                  <span>{{ gallery?.mercari_last_scraped_time ? formatDateTime(gallery.mercari_last_scraped_time) : 'Never' }}</span>
                </div>
              </div>
            </div>
            
            <div>
              <h3 class="text-lg font-semibold mb-2">Search Criteria</h3>
              <div class="grid grid-cols-1 gap-2">
                <div class="flex">
                  <span class="font-medium w-32">Keyword:</span>
                  <span>{{ gallery?.search_criteria.keyword }}</span>
                </div>
                <div class="flex">
                  <span class="font-medium w-32">Exclude:</span>
                  <span>{{ gallery?.search_criteria.exclude_keyword || 'None' }}</span>
                </div>
                <div class="flex">
                  <span class="font-medium w-32">Price Range:</span>
                  <span>
                    {{ 
                      gallery?.search_criteria.min_price ? formatPrice(gallery.search_criteria.min_price) : 'Any' 
                    }} - 
                    {{ 
                      gallery?.search_criteria.max_price ? formatPrice(gallery.search_criteria.max_price) : 'Any' 
                    }}
                  </span>
                </div>
              </div>
            </div>
          </div>
  
          <div class="mt-6">
            <h3 class="text-lg font-semibold mb-2">Evaluation Criteria</h3>
            <div v-if="gallery?.evaluation_criteria.criteria.length === 0" class="text-gray-500">
              No evaluation criteria defined
            </div>
            <div v-else class="overflow-x-auto">
              <table class="min-w-full table-auto">
                <thead>
                  <tr class="bg-gray-50 text-gray-600 text-sm">
                    <th class="py-2 px-4 text-left">Question</th>
                    <th class="py-2 px-4 text-left">Type</th>
                    <th class="py-2 px-4 text-left">Hard Criterion</th>
                  </tr>
                </thead>
                <tbody class="text-gray-600 text-sm">
                  <tr v-for="(criterion, index) in gallery?.evaluation_criteria.criteria" :key="index" class="border-b">
                    <td class="py-2 px-4">{{ criterion.question }}</td>
                    <td class="py-2 px-4">{{ criterion.criterion_type }}</td>
                    <td class="py-2 px-4">
                      <template v-if="criterion.hard_criterion">
                        <span v-if="criterion.hard_criterion.type === 'YesNo'">
                          Must be: {{ criterion.hard_criterion.value }}
                        </span>
                        <span v-else-if="criterion.hard_criterion.type === 'Int'">
                          {{ formatRangeCriterion(criterion.hard_criterion.value) }}
                        </span>
                        <span v-else-if="criterion.hard_criterion.type === 'Float'">
                          {{ formatRangeCriterion(criterion.hard_criterion.value) }}
                        </span>
                      </template>
                      <span v-else>None</span>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
  
        <!-- Sessions List -->
        <div class="bg-white shadow-md rounded-lg p-6">
          <h2 class="text-xl font-bold mb-4">Gallery Sessions</h2>
          
          <div v-if="loadingSessions" class="flex justify-center my-4">
            <div class="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-blue-500"></div>
          </div>
          
          <div v-else-if="sessionError" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            <p>{{ sessionError }}</p>
          </div>
          
          <div v-else-if="sessions.length === 0" class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded">
            <p>No sessions found for this gallery.</p>
          </div>
          
          <div v-else class="overflow-x-auto">
            <table class="min-w-full table-auto">
              <thead>
                <tr class="bg-gray-100 text-gray-600 uppercase text-sm leading-normal">
                  <th @click="sortSessionsBy('id')" class="py-3 px-6 text-left cursor-pointer">
                    Session ID
                    <span v-if="sessionSortColumn === 'id'" class="ml-1">{{ sessionSortDirection === 'asc' ? '↑' : '↓' }}</span>
                  </th>
                  <th @click="sortSessionsBy('created')" class="py-3 px-6 text-left cursor-pointer">
                    Created
                    <span v-if="sessionSortColumn === 'created'" class="ml-1">{{ sessionSortDirection === 'asc' ? '↑' : '↓' }}</span>
                  </th>
                  <th @click="sortSessionsBy('items')" class="py-3 px-6 text-left cursor-pointer">
                    Total Items
                    <span v-if="sessionSortColumn === 'items'" class="ml-1">{{ sessionSortDirection === 'asc' ? '↑' : '↓' }}</span>
                  </th>
                </tr>
              </thead>
              <tbody class="text-gray-600 text-sm">
                <tr v-for="session in sortedSessions" :key="session.id" 
                    @click="navigateToSession(session.id)"
                    class="border-b border-gray-200 hover:bg-gray-100 cursor-pointer">
                  <td class="py-3 px-6 text-left">{{ session.id }}</td>
                  <td class="py-3 px-6 text-left">{{ formatUnixTimestamp(session.stats.created) }}</td>
                  <td class="py-3 px-6 text-left">{{ session.stats.total_items }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </template>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed, onMounted } from 'vue';
  import { useRoute, useRouter } from 'vue-router';
  import { fetchGallery, fetchAllSessionStats } from '@/services/api';
  import { formatUnixTimestamp, formatPrice } from '@/utils/formatters';
  import type { Gallery, SessionId, GallerySessionStats } from '@/types/galleries';
  
  interface SessionWithStats {
    id: SessionId;
    stats: GallerySessionStats;
  }
  
  const route = useRoute();
  const router = useRouter();
  const galleryId = computed(() => route.params.id as string);
  
  const gallery = ref<Gallery | null>(null);
  const loading = ref(true);
  const error = ref<string | null>(null);
  
  const sessions = ref<SessionWithStats[]>([]);
  const loadingSessions = ref(true);
  const sessionError = ref<string | null>(null);
  const sessionSortColumn = ref('created');
  const sessionSortDirection = ref<'asc' | 'desc'>('desc');
  
  onMounted(async () => {
    await fetchGalleryData();
    await fetchSessionData();
  });
  
  async function fetchGalleryData() {
    loading.value = true;
    error.value = null;
    
    try {
      gallery.value = await fetchGallery(galleryId.value);
    } catch (err) {
      error.value = 'Failed to load gallery data. Please try again later.';
      console.error(err);
    } finally {
      loading.value = false;
    }
  }
  
  async function fetchSessionData() {
    loadingSessions.value = true;
    sessionError.value = null;
    
    try {
      const data = await fetchAllSessionStats(galleryId.value);
      sessions.value = data.map(([id, stats]) => ({ id, stats }));
    } catch (err) {
      sessionError.value = 'Failed to load session data. Please try again later.';
      console.error(err);
    } finally {
      loadingSessions.value = false;
    }
  }
  
  function formatDateTime(dateTime: string): string {
    return new Date(dateTime).toLocaleString();
  }
  
  function formatRangeCriterion(criterion: { min?: number; max?: number }): string {
    if (criterion.min !== undefined && criterion.max !== undefined) {
      return `Range: ${criterion.min} - ${criterion.max}`;
    } else if (criterion.min !== undefined) {
      return `Min: ${criterion.min}`;
    } else if (criterion.max !== undefined) {
      return `Max: ${criterion.max}`;
    }
    return 'No range specified';
  }
  
  function sortSessionsBy(column: string) {
    if (sessionSortColumn.value === column) {
      sessionSortDirection.value = sessionSortDirection.value === 'asc' ? 'desc' : 'asc';
    } else {
      sessionSortColumn.value = column;
      sessionSortDirection.value = 'desc';
    }
  }
  
  const sortedSessions = computed(() => {
    return [...sessions.value].sort((a, b) => {
      const direction = sessionSortDirection.value === 'asc' ? 1 : -1;
      
      switch (sessionSortColumn.value) {
        case 'id':
          return direction * (a.id - b.id);
        case 'created':
          return direction * (a.stats.created - b.stats.created);
        case 'items':
          return direction * (a.stats.total_items - b.stats.total_items);
        default:
          return 0;
      }
    });
  });
  
  function navigateToSession(sessionId: SessionId) {
    router.push(`/session/${sessionId}`);
  }
  </script>