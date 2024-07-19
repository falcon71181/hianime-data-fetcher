use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};

use crate::schema::anime;

#[derive(Insertable, Queryable, Debug, AsChangeset)]
#[diesel(table_name = anime)]
pub struct Anime {
    pub id: i32,
    pub title: String,
    pub description: String,
}
