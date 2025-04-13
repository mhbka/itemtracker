// @generated automatically by Diesel CLI.

diesel::table! {
    embedded_marketplace_items (id) {
        id -> Int4,
        gallery_session_id -> Int4,
        marketplace_item_id -> Int4,
        item_description -> Text,
        description_embedding -> Array<Nullable<Float4>>,
        image_embedding -> Array<Nullable<Float4>>,
        evaluation_answers -> Array<Nullable<Jsonb>>,
    }
}

diesel::table! {
    galleries (id) {
        id -> Uuid,
        user_id -> Uuid,
        scraping_periodicity -> Text,
        search_criteria -> Jsonb,
        evaluation_criteria -> Jsonb,
        mercari_last_scraped_time -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    gallery_sessions (id) {
        id -> Int4,
        gallery_id -> Uuid,
        created -> Timestamp,
        used_evaluation_criteria -> Jsonb,
    }
}

diesel::table! {
    marketplace_items (id) {
        id -> Int4,
        marketplace -> Varchar,
        item_id -> Varchar,
        name -> Varchar,
        price -> Float8,
        description -> Text,
        status -> Varchar,
        category -> Varchar,
        thumbnails -> Array<Nullable<Text>>,
        item_condition -> Varchar,
        created -> Timestamp,
        updated -> Timestamp,
        seller_id -> Varchar,
    }
}

diesel::joinable!(embedded_marketplace_items -> gallery_sessions (gallery_session_id));
diesel::joinable!(embedded_marketplace_items -> marketplace_items (marketplace_item_id));
diesel::joinable!(gallery_sessions -> galleries (gallery_id));

diesel::allow_tables_to_appear_in_same_query!(
    embedded_marketplace_items,
    galleries,
    gallery_sessions,
    marketplace_items,
);
