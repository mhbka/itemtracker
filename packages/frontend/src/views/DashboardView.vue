<template>
  <div class="dashboard container mx-auto py-8 px-4">
    <h1 class="text-2xl font-bold mb-6">Galleries Dashboard</h1>
    
    <div v-if="loading" class="flex justify-center my-8">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"></div>
    </div>
    
    <div v-else-if="error" class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
      <p>{{ error }}</p>
    </div>
    
    <div v-else-if="galleries.length === 0" class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded">
      <p>No galleries found. Create a new gallery to get started.</p>
    </div>
    
    <div v-else class="overflow-x-auto bg-white shadow-md rounded-lg">
      <table class="min-w-full table-auto">
        <thead>
          <tr class="bg-gray-100 text-gray-600 uppercase text-sm leading-normal">
            <th @click="sortBy('id')" class="py-3 px-6 text-left cursor-pointer">
              Gallery ID
              <span v-if="sortColumn === 'id'" class="ml-1">{{ sortDirection === 'asc' ? '↑' : '↓' }}</span>
            </th>
            <th @click="sortBy('sessions')" class="py-3 px-6 text-left cursor-pointer">
              Total Sessions
              <span v-if="sortColumn === 'sessions'" class="ml-1">{{ sortDirection === 'asc' ? '↑' : '↓' }}</span>
            </th>
            <th @click="sortBy('items')" class="py-3 px-6 text-left cursor-pointer">
              Total Items
              <span v-if="sortColumn === 'items'" class="ml-1">{{ sortDirection === 'asc' ? '↑' : '↓' }}</span>
            </th>
            <th @click="sortBy('lastScrape')" class="py-3 px-6 text-left cursor-pointer">
              Last Scraped
              <span v-if="sortColumn === 'lastScrape'" class="ml-1">{{ sortDirection === 'asc' ? '↑' : '↓' }}</span>
            </th>
          </tr>
        </thead>
        <tbody class="text-gray-600 text-sm">
          <tr v-for="gallery in sortedGalleries" :key="gallery.id" 
              @click="navigateToGallery(gallery.id)" 
              class="border-b border-gray-200 hover:bg-gray-100 cursor-pointer">
            <td class="py-3 px-6 text-left">{{ truncatedId(gallery.id) }}</td>
            <td class="py-3 px-6 text-left">{{ gallery.stats.total_sessions }}</td>
            <td class="py-3 px-6 text-left">{{ gallery.stats.total_mercari_items }}</td>
            <td class="py-3 px-6 text-left">{{ formatUnixTimestamp(gallery.stats.latest_scrape) }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { fetchAllGalleryStats } from '@/services/api';
import { formatUnixTimestamp } from '@/utils/formatters';
import type { UUID, GalleryStats } from '@/types/galleries';

interface GalleryWithStats {
  id: UUID;
  stats: GalleryStats;
}

const router = useRouter();
const galleries = ref<GalleryWithStats[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);
const sortColumn = ref('lastScrape');
const sortDirection = ref<'asc' | 'desc'>('desc');

onMounted(async () => {
  try {
    const data = await fetchAllGalleryStats();
    galleries.value = data.map(([id, stats]) => ({ id, stats }));
  } catch (err) {
    error.value = 'Failed to load gallery data. Please try again later.';
    console.error(err);
  } finally {
    loading.value = false;
  }
});

const truncatedId = (id: UUID) => {
  return id.substring(0, 8) + '...';
};

const sortBy = (column: string) => {
  if (sortColumn.value === column) {
    sortDirection.value = sortDirection.value === 'asc' ? 'desc' : 'asc';
  } else {
    sortColumn.value = column;
    sortDirection.value = 'desc';
  }
};

const sortedGalleries = computed(() => {
  return [...galleries.value].sort((a, b) => {
    const direction = sortDirection.value === 'asc' ? 1 : -1;
    
    switch (sortColumn.value) {
      case 'id':
        return direction * a.id.localeCompare(b.id);
      case 'sessions':
        return direction * (a.stats.total_sessions - b.stats.total_sessions);
      case 'items':
        return direction * (a.stats.total_mercari_items - b.stats.total_mercari_items);
      case 'lastScrape':
        return direction * (a.stats.latest_scrape - b.stats.latest_scrape);
      default:
        return 0;
    }
  });
});

const navigateToGallery = (galleryId: UUID) => {
  router.push(`/gallery/${galleryId}`);
};
</script>