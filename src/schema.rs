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
        #[max_length = 500]
        synonyms -> Nullable<Varchar>,
        #[max_length = 200]
        image -> Varchar,
        #[max_length = 50]
        category -> Varchar,
        #[max_length = 50]
        rating -> Varchar,
        #[max_length = 50]
        quality -> Varchar,
        #[max_length = 50]
        duration -> Varchar,
        #[max_length = 100]
        premiered -> Varchar,
        #[max_length = 100]
        aired -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        #[max_length = 50]
        mal_score -> Varchar,
        studios -> Text,
        producers -> Text,
        genres -> Text,
        sub_episodes -> Int4,
        dub_episodes -> Int4,
        total_episodes -> Int4,
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

diesel::table! {
    anime_staff (anime_id, staff_id) {
        anime_id -> Int4,
        staff_id -> Int4,
        positions -> Array<Nullable<Text>>,
    }
}

diesel::table! {
    episodes (id) {
        #[max_length = 500]
        id -> Varchar,
        episode_no -> Int4,
        #[max_length = 500]
        title -> Varchar,
        is_filler -> Bool,
        anime_id -> Int4,
    }
}

diesel::table! {
    staff (mal_id) {
        mal_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 500]
        mal_url -> Varchar,
        #[max_length = 200]
        image -> Varchar,
        positions -> Array<Nullable<Text>>,
    }
}

diesel::joinable!(anime_staff -> anime (anime_id));
diesel::joinable!(anime_staff -> staff (staff_id));
diesel::joinable!(episodes -> anime (anime_id));

diesel::allow_tables_to_appear_in_same_query!(
    anime,
    anime_id,
    anime_staff,
    episodes,
    staff,
);
