// @generated automatically by Diesel CLI.

diesel::table! {
    anime (id) {
        id -> Int4,
        #[max_length = 500]
        title -> Varchar,
        description -> Text,
        mal_id -> Int4,
        al_id -> Int4,
        #[max_length = 500]
        japanese_title -> Nullable<Varchar>,
        #[max_length = 200]
        image -> Varchar,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Varchar,
        #[max_length = 50]
        sub_or_dub -> Varchar,
    }
}

diesel::table! {
    anime_id (id) {
        id -> Int4,
        #[max_length = 500]
        anime_name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    anime,
    anime_id,
);
