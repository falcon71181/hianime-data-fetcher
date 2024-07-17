use diesel::prelude::*;

use crate::schema::animes;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = animes)]
pub struct Anime {
    pub id: String,
    pub title: String,
    pub description: String,
}

// #[derive(Queryable, Debug)]
// pub struct EpisodeInfo {
//     pub anime_id: String,
//     pub sub: i32,
//     pub dub: i32,
//     pub eps: i32,
// }
