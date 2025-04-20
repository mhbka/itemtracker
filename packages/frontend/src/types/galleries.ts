import { CriterionAnswer, EvaluationCriteria } from './evaluationCriteria'
import { SearchCriteria } from './searchCriteria'

export type UUID = string
export type SessionId = number
export type GalleryId = UUID
export type ItemId = string
export type UnixUtcDateTime = number
export type ValidCronString = string
export type NaiveDateTime = string

// Summary stats for a gallery.
export interface GalleryStats {
  name: string
  total_sessions: number
  total_mercari_items: number
  latest_scrape?: UnixUtcDateTime
}

// Maps a gallery's ID to its stats.
export interface GalleryListItem {
  id: UUID
  stats: GalleryStats
}

// For submitting a new gallery to be registered.
export interface NewGallery {
  name: string
  is_active: boolean
  scraping_periodicity: ValidCronString
  search_criteria: SearchCriteria
  evaluation_criteria: EvaluationCriteria
  mercari_last_scraped_time: NaiveDateTime
}

// Data for a gallery.
export interface Gallery {
  id: UUID
  user_id: UUID
  name: string
  is_active: boolean
  scraping_periodicity: ValidCronString
  search_criteria: SearchCriteria
  evaluation_criteria: EvaluationCriteria
  mercari_last_scraped_time?: NaiveDateTime
  created_at: NaiveDateTime
  updated_at: NaiveDateTime
}

// Summary statistics for a gallery session.
export interface GallerySessionStats {
  created: UnixUtcDateTime
  total_items: number
}

// Maps a gallery session to its stats.
export interface SessionListItem {
  id: SessionId
  stats: GallerySessionStats
}

// The raw data of the item listing.
export interface MarketplaceItemData {
  item_id: ItemId
  name: string
  price: number
  description: string
  status: string
  seller_id: string
  category: string
  thumbnails: string[]
  item_condition: string
  created: UnixUtcDateTime
  updated: UnixUtcDateTime
}

/// A single scraped + processed item listing.
export interface MarketplaceItem {
  item: MarketplaceItemData
  evaluation_answers: CriterionAnswer[]
  item_description: string
}

/// All data under a gallery session.
export interface GallerySession {
  id: SessionId
  gallery_id: GalleryId
  created: UnixUtcDateTime
  used_evaluation_criteria: EvaluationCriteria
  mercari_items: MarketplaceItem[]
}
