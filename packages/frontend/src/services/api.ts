import type {
  UUID,
  GalleryStats,
  Gallery,
  SessionId,
  GallerySessionStats,
  GallerySession,
  NewGallery,
} from '@/types/galleries'
import apiClient from './client'

export async function fetchAllGalleryStats(): Promise<[UUID, GalleryStats][]> {
  const response = await apiClient.get('/g/stats/all')
  return response.data
}

export async function fetchGallery(galleryId: UUID): Promise<Gallery> {
  const response = await apiClient.get(`/g/${galleryId}`)
  return response.data
}

export async function fetchAllSessionStats(
  galleryId: UUID,
): Promise<[SessionId, GallerySessionStats][]> {
  const response = await apiClient.get(`/s/stats/all/${galleryId}`)
  return response.data
}

export async function fetchSession(sessionId: SessionId): Promise<GallerySession> {
  const response = await apiClient.get(`/s/${sessionId}`)
  return response.data
}

export async function addNewGallery(gallery: NewGallery): Promise<void> {
  await apiClient.post(`/g`, gallery)
}

export async function deleteGallery(galleryId: UUID): Promise<void> {
  await apiClient.delete(`/g/${galleryId}`)
}

export async function resetGallery(galleryId: UUID): Promise<void> {
  await apiClient.delete(`/g/reset/${galleryId}`)
}

export async function pauseUnpauseGallery(galleryId: UUID, active: boolean): Promise<void> {
  const updatedData = { is_active: active }
  await apiClient.patch(`/g/${galleryId}`, updatedData)
}

export async function formatDate(timestamp: number): Promise<string> {
  return new Date(timestamp * 1000).toLocaleString()
}
