// @generated automatically by Diesel CLI.

diesel::table! {
    animes (id) {
        #[max_length = 50]
        id -> Varchar,
        #[max_length = 100]
        title -> Nullable<Varchar>,
        #[max_length = 500]
        description -> Nullable<Varchar>,
    }
}
