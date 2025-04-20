// The search criteria used in marketplaces to search for the initial items.
export interface SearchCriteria {
  keyword: string
  exclude_keyword: string
  min_price?: number
  max_price?: number
}
