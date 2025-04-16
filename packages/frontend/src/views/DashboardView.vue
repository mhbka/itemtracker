<template>
  <div class="dashboard-container">
    <div class="dashboard-header">
      <h1 class="dashboard-title">Galleries Dashboard</h1>
    </div>

    <div v-if="loading" class="loading-spinner-container">
      <div class="loading-spinner"></div>
    </div>

    <div v-else-if="error" class="error-message">
      <p>{{ error }}</p>
    </div>

    <div v-else-if="galleries.length === 0" class="empty-message">
      <p>No galleries found. Create a new gallery to get started.</p>
      <button @click="navigateToNewGallery" class="primary-button">Create New Gallery</button>
    </div>

    <div v-else class="table-container">
      <button @click="navigateToNewGallery" class="primary-button">Create New Gallery</button>
      <table class="gallery-table">
        <thead>
          <tr class="table-header">
            <th>
              Name
            </th>
            <th>
              Gallery ID
            </th>
            <th>
              Total Sessions
            </th>
            <th>
              Total Items
            </th>
            <th>
              Last Scraped
            </th>
          </tr>
        </thead>
        
        <tbody>
          <tr v-for="gallery in galleries" :key="gallery.id" @click="navigateToGallery(gallery.id)">
            <td>{{ gallery.stats.name }}</td>
            <td>{{ gallery.id }}</td>
            <td>{{ gallery.stats.total_sessions }}</td>
            <td>{{ gallery.stats.total_mercari_items }}</td>
            <td>{{ formatUnixTimestamp(gallery.stats.latest_scrape) }}</td>
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
import { type UUID, type GalleryStats } from '@/types/galleries';

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

const navigateToGallery = (galleryId: UUID) => {
  router.push(`/gallery/${galleryId}`);
};

const navigateToNewGallery = () => {
  router.push('/new_gallery');
};
</script>

<style scoped>
.dashboard-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem 1rem;
}

.dashboard-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.dashboard-title {
  font-size: 1.5rem;
  font-weight: bold;
  margin: 0;
}

.loading-spinner-container {
  display: flex;
  justify-content: center;
  margin: 2rem 0;
}

.loading-spinner {
  width: 3rem;
  height: 3rem;
  border: 0.25rem solid #3b82f6;
  border-top-color: transparent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.error-message,
.empty-message {
  background-color: #fef2f2;
  border: 1px solid #fca5a5;
  color: #b91c1c;
  padding: 1rem;
  border-radius: 0.5rem;
  margin-bottom: 1rem;
}

.empty-message {
  background-color: #fef9c3;
  border-color: #facc15;
  color: #92400e;
  text-align: center;
  padding: 2rem;
}

.primary-button {
  padding: 0.5rem 1rem;
  margin-top: 1rem;
}

.primary-button:hover {
  background-color: #2563eb;
}

.table-container {
  overflow-x: auto;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.05);
  border-radius: 0.5rem;
}

.gallery-table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 1rem;
}

.table-header {
  text-transform: uppercase;
  font-size: 0.875rem;
  font-weight: 600;
}

.table-header th {
  padding: 0.75rem 1.5rem;
  text-align: left;
  cursor: pointer;
}

.sortable {
  user-select: none;
}
</style>