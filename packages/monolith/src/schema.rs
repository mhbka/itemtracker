// @generated automatically by Diesel CLI.

diesel::table! {
    galleries (id) {
        id -> Uuid,
        user_id -> Uuid,
        scraping_periodicity -> Text,
        search_criteria -> Jsonb,
        evaluation_criteria -> Jsonb,
        mercari_last_scraped_time -> Nullable<Int8>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
