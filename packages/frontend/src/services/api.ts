import type { 
  UUID, 
  GalleryStats, 
  Gallery, 
  SessionId, 
  GallerySessionStats, 
  GallerySession 
} from '@/types/galleries';
import apiClient from './client';

export async function fetchAllGalleryStats(): Promise<[UUID, GalleryStats][]> {
  const response = await apiClient.get('/g/stats/all');
  return response.data;
}

export async function fetchGallery(galleryId: UUID): Promise<Gallery> {
  const response = await apiClient.get(`/g/${galleryId}`);
  return response.data;
}

export async function fetchAllSessionStats(galleryId: UUID): Promise<[SessionId, GallerySessionStats][]> {
  const response = await apiClient.get(`/s/stats/all?gallery_id=${galleryId}`);
  return response.data;
}

export async function fetchSession(sessionId: SessionId): Promise<GallerySession> {
  const response = await apiClient.get(`/s/${sessionId}`);
  return response.data;
}

export async function formatDate(timestamp: number): Promise<string> {
  return new Date(timestamp * 1000).toLocaleString();
}