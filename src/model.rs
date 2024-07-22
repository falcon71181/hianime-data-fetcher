use diesel::prelude::*;
use serde::Deserialize;

use crate::schema::{anime, anime_id};

#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = anime)]
pub struct Anime {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub mal_id: i32,
    pub al_id: i32,
    pub japanese_title: Option<String>,
    pub image: String,
    pub type_: String,
    pub sub_or_dub: String,
}

#[derive(Queryable, Insertable, Selectable, Debug, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = anime_id)]
pub struct AnimeID {
    pub anime_name: String,
}
