// @generated automatically by Diesel CLI.

diesel::table! {
    anime (id) {
        id -> Int4,
        #[max_length = 100]
        title -> Nullable<Varchar>,
        #[max_length = 500]
        description -> Nullable<Varchar>,
    }
}
