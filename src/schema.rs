// @generated automatically by Diesel CLI.

diesel::table! {
    anime (id) {
        id -> Int4,
        #[max_length = 200]
        title -> Varchar,
        description -> Text,
        mal_id -> Int4,
        al_id -> Int4,
        #[max_length = 200]
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
