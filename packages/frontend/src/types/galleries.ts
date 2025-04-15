export type UUID = string;
export type SessionId = number;
export type GalleryId = UUID;
export type ItemId = string;
export type UnixUtcDateTime = number;
export type ValidCronString = string;
export type NaiveDateTime = string;

export interface GalleryStats {
  total_sessions: number;
  total_mercari_items: number;
  latest_scrape: UnixUtcDateTime;
}

export interface GalleryListItem {
  id: UUID;
  stats: GalleryStats;
}

export interface SearchCriteria {
  keyword: string;
  exclude_keyword: string;
  min_price?: number;
  max_price?: number;
}

export enum CriterionType {
  YesNo = "YesNo",
  YesNoUncertain = "YesNoUncertain",
  Int = "Int",
  Float = "Float",
  OpenEnded = "OpenEnded"
}

export enum YesNo {
  Yes = "Yes",
  No = "No"
}

export enum YesNoUncertain {
  Yes = "Yes",
  No = "No",
  Uncertain = "Uncertain"
}

export interface IntHardCriterion {
  min?: number;
  max?: number;
}

export interface FloatHardCriterion {
  min?: number;
  max?: number;
}

export type HardCriterion = 
  | { type: "YesNo"; value: YesNo }
  | { type: "Int"; value: IntHardCriterion }
  | { type: "Float"; value: FloatHardCriterion };

export interface Criterion {
  question: string;
  criterion_type: CriterionType;
  hard_criterion?: HardCriterion;
}

export interface EvaluationCriteria {
  criteria: Criterion[];
}

export interface Gallery {
  id: UUID;
  user_id: UUID;
  name: string;
  is_active: boolean;
  scraping_periodicity: ValidCronString;
  search_criteria: SearchCriteria;
  evaluation_criteria: EvaluationCriteria;
  mercari_last_scraped_time?: NaiveDateTime;
  created_at: NaiveDateTime;
  updated_at: NaiveDateTime;
}

export interface GallerySessionStats {
  created: UnixUtcDateTime;
  total_items: number;
}

export interface SessionListItem {
  id: SessionId;
  stats: GallerySessionStats;
}

export type CriterionAnswer = 
  | { type: "YesNo"; value: YesNo }
  | { type: "YesNoUncertain"; value: YesNoUncertain }
  | { type: "Int"; value: number }
  | { type: "Float"; value: number }
  | { type: "OpenEnded"; value: string };

export interface MarketplaceItemData {
  item_id: ItemId;
  name: string;
  price: number;
  description: string;
  status: string;
  seller_id: string;
  category: string;
  thumbnails: string[];
  item_condition: string;
  created: UnixUtcDateTime;
  updated: UnixUtcDateTime;
}

export interface EmbeddedMarketplaceItem {
  item: MarketplaceItemData;
  evaluation_answers: CriterionAnswer[];
  item_description: string;
  description_embedding: number[];
  image_embedding: number[];
}

export interface GallerySession {
  id: SessionId;
  gallery_id: GalleryId;
  created: UnixUtcDateTime;
  used_evaluation_criteria: EvaluationCriteria;
  mercari_items: EmbeddedMarketplaceItem[];
}