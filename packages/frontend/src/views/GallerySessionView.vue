<template>
  <div class="session-view">
    <h1>Session Details</h1>
    <div class="back-nav">
      <button @click="navigateBack" class="back-button">← Back to Gallery</button>
    </div>

    <div v-if="loading" class="loading-spinner-container">
      <div class="loading-spinner"></div>
    </div>

    <div v-else-if="error" class="error-message">
      <p>{{ error }}</p>
    </div>

    <template v-else>
      <!-- Session Information -->
      <div class="card">
        <div class="info-grid">
          <div class="info-row">
            <span class="label">Gallery ID:</span>
            <span>{{ session?.gallery_id }}</span>
          </div>
          <div class="info-row">
            <span class="label">Created:</span>
            <span>{{ formatUnixTimestamp(session?.created || 0) }}</span>
          </div>
          <div class="info-row">
            <span class="label">Total Items:</span>
            <span>{{ session?.mercari_items.length || 0 }}</span>
          </div>
        </div>
      </div>

      <!-- Items Table -->
      <div class="card">
        <h2>Marketplace Items</h2>

        <div v-if="session?.mercari_items.length === 0" class="empty-message">
          <p>No items found in this session.</p>
        </div>

        <div v-else class="items-table-container">
          <table class="items-table">
            <thead>
              <tr>
                <th>Image</th>
                <th>ID</th>
                <th>Name</th>
                <th>Description</th>
                <th>Price</th>
                <!-- the below columns aren't used
                <th>Status</th>
                <th>Seller ID</th>
                <th>Category</th>
                <th>Condition</th>
                <th>Created</th>
                <th>Updated</th>
                -->
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="(item, index) in session?.mercari_items"
                :key="item.item.item_id"
                class="item-row"
              >
                <td class="item-image">
                  <img
                    v-if="item.item.thumbnails?.length > 0"
                    :src="item.item.thumbnails[0]"
                    :alt="item.item.name"
                    class="thumbnail"
                  />
                  <div v-else class="no-thumbnail">
                    <span>No image</span>
                  </div>
                </td>
                <td class="item-column">{{ item.item.item_id }}</td>
                <td class="item-column">{{ item.item.name }}</td>
                <td class="item-column">{{ item.item_description }}</td>
                <td class="item-column">{{ formatPrice(item.item.price) }}</td>
                <!-- as said above
                <td class="item-column">{{ item.item.status }}</td>
                <td class="item-column">{{ item.item.seller_id }}</td>
                <td class="item-column">{{ item.item.category }}</td>
                <td class="item-column">{{ item.item.item_condition }}</td>
                <td class="item-column">{{ formatUnixTimestamp(item.item.created) }}</td>
                <td class="item-column">{{ formatUnixTimestamp(item.item.updated) }}</td>
                -->

                <!--
                <div v-if="item.evaluation_answers.length > 0" class="section">
                  <h4>Evaluation Answers:</h4>
                  <div>
                    <div v-for="(answer, ansIndex) in item.evaluation_answers" :key="ansIndex" class="evaluation-answer">
                      <span class="answer-question">
                        {{ session?.used_evaluation_criteria.criteria[ansIndex]?.question || `Question ${ansIndex + 1}` }}:
                      </span>
                      <span>{{ formatCriterionAnswer(answer) }}</span>
                    </div>
                  </div>
                </div>
                -->
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { fetchSession } from '@/services/api'
import { formatUnixTimestamp, formatPrice, formatCriterionAnswer } from '@/utils/formatters'
import type { GallerySession } from '@/types/galleries'

const route = useRoute()
const router = useRouter()
const sessionId = ref(parseInt(route.params.id as string, 10))
const expandedItems = ref<number[]>([])

const session = ref<GallerySession | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)

onMounted(async () => {
  await fetchSessionData()
})

async function fetchSessionData() {
  loading.value = true
  error.value = null

  try {
    session.value = await fetchSession(sessionId.value)
  } catch (err) {
    error.value = 'Failed to load session data. Please try again later.'
    console.error(err)
  } finally {
    loading.value = false
  }
}

function navigateBack() {
  if (session.value?.gallery_id) {
    router.push(`/gallery/${session.value.gallery_id}`)
  } else {
    router.push('/dashboard')
  }
}

function toggleItemDetails(index: number) {
  const currentIndex = expandedItems.value.indexOf(index)
  if (currentIndex === -1) {
    expandedItems.value.push(index)
  } else {
    expandedItems.value.splice(currentIndex, 1)
  }
}
</script>

<style scoped>
.session-view {
  max-width: 2000px;
  margin: 0 auto;
}

.back-nav {
  margin-bottom: 1rem;
}

.loading-spinner-container {
  display: flex;
  justify-content: center;
  margin: 2rem 0;
}

.loading-spinner {
  width: 3rem;
  height: 3rem;
  border: 0.25rem solid #ccc;
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
  border: 1px solid #ccc;
  padding: 1rem;
  margin-bottom: 1rem;
}

.card {
  border: 1px solid #ccc;
  margin-bottom: 1rem;
  padding: 1rem;
  border-radius: 0.5rem;
}

.info-row {
  display: flex;
  margin: 0.5rem 0;
}

.label {
  font-weight: bold;
  width: 8rem;
}

.section {
  margin-top: 1rem;
}

.section h4 {
  font-weight: bold;
  margin-bottom: 0.5rem;
}

.evaluation-answer {
  margin-bottom: 0.5rem;
}

.answer-question {
  font-weight: bold;
}

.thumbnail,
.no-thumbnail {
  width: 150px; /* Fixed width */
  height: 150px; /* Same as width to make it square */
  min-width: 150px; /* Prevent shrinking */
  border: 1px solid #ccc;
  object-fit: cover; /* Makes image cover the square area */
  aspect-ratio: 1/1; /* Enforces square aspect ratio */
}

.no-thumbnail {
  display: flex;
  align-items: center;
  justify-content: center;
}

.items-table {
  border-collapse: collapse;
}

.item-image {
  width: 100px;
}
</style>
