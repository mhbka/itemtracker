<template>
  <div class="gallery-view">
    <div v-if="loading" class="loading-container">
      <div class="loading-spinner"></div>
    </div>

    <div v-else-if="error" class="error-message">
      <p>{{ error }}</p>
      <button @click="$router.push('/dashboard')" class="primary-button">
        <span class="back-arrow">←</span> Back to Dashboard
      </button>
    </div>

    <div v-else>
      <button @click="$router.push('/dashboard')" class="primary-button">
        <span class="back-arrow">←</span> Back to Dashboard
      </button>

      <div class="details-card">
        <h1 class="details-title">{{ gallery?.name }}</h1>

        <button @click="submitDeleteGallery" class="primary-button">Delete Gallery</button>

        <div class="info-grid">
          <div class="info-section">
            <h3 class="section-title">Basic Information</h3>
            <div class="info-list">
              <div class="info-item">
                <span class="info-label">ID:</span>
                <span>{{ gallery?.id }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">Status:</span>
                <span :class="gallery?.is_active ? 'status-active' : 'status-inactive'">
                  {{ gallery?.is_active ? 'Active' : 'Inactive' }}
                </span>
              </div>
              <div class="info-item">
                <span class="info-label">Cron Schedule:</span>
                <span>{{ gallery?.scraping_periodicity }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">Last Scraped:</span>
                <span>
                  {{
                    gallery?.mercari_last_scraped_time &&
                    gallery?.mercari_last_scraped_time != getZeroedNaiveDatetime()
                      ? formatDateTime(gallery.mercari_last_scraped_time)
                      : 'Never'
                  }}
                </span>
              </div>
            </div>
          </div>

          <div class="info-section">
            <h3 class="section-title">Search Criteria</h3>
            <div class="info-list">
              <div class="info-item">
                <span class="info-label">Keywords:</span>
                <span>{{ gallery?.search_criteria.keyword }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">Excluded Keywords:</span>
                <span>{{ gallery?.search_criteria.exclude_keyword || '-' }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">Price Range:</span>
                <span>
                  {{
                    gallery?.search_criteria.min_price
                      ? formatPrice(gallery.search_criteria.min_price)
                      : '-'
                  }}
                  -
                  {{
                    gallery?.search_criteria.max_price
                      ? formatPrice(gallery.search_criteria.max_price)
                      : '-'
                  }}
                </span>
              </div>
            </div>
          </div>
        </div>

        <div class="criteria-section">
          <h3 class="section-title">Evaluation Criteria</h3>
          <div
            v-if="gallery?.evaluation_criteria.criteria.length === 0"
            class="no-criteria-message"
          >
            No evaluation criteria defined
          </div>
          <div v-else class="table-container">
            <table class="criteria-table">
              <thead>
                <tr>
                  <th>Question</th>
                  <th>Type</th>
                  <th>Hard Criterion</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="(criterion, index) in gallery?.evaluation_criteria.criteria"
                  :key="index"
                >
                  <td>{{ criterion.question }}</td>
                  <td>{{ criterion.criterion_type }}</td>
                  <td>
                    <span v-if="criterion.hard_criterion">{{
                      formatHardCriterion(criterion.hard_criterion)
                    }}</span>
                    <span v-else>No</span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- Sessions List -->
      <div class="sessions-card">
        <h2 class="sessions-title">Gallery Sessions</h2>

        <div v-if="loadingSessions" class="loading-container">
          <div class="loading-spinner small"></div>
        </div>

        <div v-else-if="sessionError" class="error-message">
          <p>{{ sessionError }}</p>
        </div>

        <div v-else-if="sessions.length === 0" class="no-sessions-message">
          <p>No sessions found for this gallery.</p>
        </div>

        <div v-else class="table-container">
          <table class="sessions-table">
            <thead>
              <tr>
                <th>Session ID</th>
                <th>Created</th>
                <th>Total Items</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="session in sessions"
                :key="session.id"
                @click="navigateToSession(session.id)"
                class="session-row"
              >
                <td>{{ session.id }}</td>
                <td>{{ formatUnixTimestamp(session.stats.created) }}</td>
                <td>{{ session.stats.total_items }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { fetchGallery, fetchAllSessionStats, deleteGallery } from '@/services/api'
import {
  formatUnixTimestamp,
  formatPrice,
  getZeroedNaiveDatetime,
  formatCriterionAnswer,
  formatHardCriterion,
} from '@/utils/formatters'
import type { Gallery, SessionId, GallerySessionStats } from '@/types/galleries'

interface SessionWithStats {
  id: SessionId
  stats: GallerySessionStats
}

const route = useRoute()
const router = useRouter()
const galleryId = computed(() => route.params.id as string)

const gallery = ref<Gallery | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)

const sessions = ref<SessionWithStats[]>([])
const loadingSessions = ref(true)
const sessionError = ref<string | null>(null)

onMounted(async () => {
  await fetchGalleryData()
  await fetchSessionData()
})

async function fetchGalleryData() {
  loading.value = true
  error.value = null

  try {
    gallery.value = await fetchGallery(galleryId.value)
  } catch (err) {
    error.value = 'Failed to load gallery data. Please try again later.'
    console.error(err)
  } finally {
    loading.value = false
  }
}

async function fetchSessionData() {
  loadingSessions.value = true
  sessionError.value = null

  try {
    const data = await fetchAllSessionStats(galleryId.value)
    sessions.value = data.map(([id, stats]) => ({ id, stats }))
  } catch (err) {
    sessionError.value = 'Failed to load session data. Please try again later.'
    console.error(err)
  } finally {
    loadingSessions.value = false
  }
}

async function submitDeleteGallery() {
  try {
    await deleteGallery(galleryId.value)
    router.push('/dashboard')
  } catch (err) {
    sessionError.value = 'Failed to delete gallery. Please try again later.'
    console.error(err)
  }
}

function formatDateTime(dateTime: string): string {
  return new Date(dateTime).toLocaleString()
}

function formatRangeCriterion(criterion: { min?: number; max?: number }): string {
  if (criterion.min !== undefined && criterion.max !== undefined) {
    return `Range: ${criterion.min} - ${criterion.max}`
  } else if (criterion.min !== undefined) {
    return `Min: ${criterion.min}`
  } else if (criterion.max !== undefined) {
    return `Max: ${criterion.max}`
  }
  return 'No range specified'
}

function navigateToSession(sessionId: SessionId) {
  router.push(`/session/${sessionId}`)
}
</script>

<style scoped>
.gallery-view {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 1rem;
}

.galleries-container {
  padding: 1.5rem;
  border: 1px solid #e5e7eb;
  border-radius: 0.5rem;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.05);
}

.loading-container {
  display: flex;
  justify-content: center;
  margin: 2rem 0;
}

.loading-spinner {
  height: 3rem;
  width: 3rem;
  border-radius: 50%;
  border-top: 2px solid #2563eb;
  border-bottom: 2px solid #2563eb;
  border-left: 2px solid transparent;
  border-right: 2px solid transparent;
  animation: spin 1s linear infinite;
}

.loading-spinner.small {
  height: 2rem;
  width: 2rem;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

.error-message {
  background-color: #fee2e2;
  border: 1px solid #f87171;
  border-radius: 0.375rem;
  color: #b91c1c;
  padding: 0.75rem 1rem;
  margin-bottom: 1.5rem;
}

.primary-button {
  margin: 0.5rem 0rem;
}

.back-arrow {
  margin-right: 0.5rem;
}

.details-card,
.sessions-card {
  padding: 1.5rem;
  border: 1px solid #e5e7eb;
  border-radius: 0.5rem;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.05);
}

.details-card {
  margin-bottom: 2rem;
}

.details-title {
  font-size: 1.5rem;
  font-weight: bold;
  margin-bottom: 1rem;
}

.info-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1.5rem;
}

@media (min-width: 768px) {
  .info-grid {
    grid-template-columns: 1fr 1fr;
  }
}

.section-title {
  font-size: 1.125rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
}

.info-list {
  display: grid;
  grid-template-columns: 1fr;
  gap: 0.5rem;
}

.info-item {
  display: flex;
}

.info-label {
  font-weight: 500;
  width: 8rem;
}

.status-active {
  color: #16a34a;
}

.status-inactive {
  color: #dc2626;
}

.criteria-section {
  margin-top: 1.5rem;
}

.no-criteria-message {
  color: #6b7280;
}

.table-container {
  overflow-x: auto;
}

.criteria-table,
.sessions-table {
  min-width: 100%;
  border-collapse: collapse;
}

.sessions-title {
  font-size: 1.25rem;
  font-weight: bold;
  margin-bottom: 1rem;
}

.no-sessions-message {
  background-color: #fef3c7;
  border: 1px solid #fbbf24;
  border-radius: 0.375rem;
  color: #92400e;
  padding: 0.75rem 1rem;
}
</style>
