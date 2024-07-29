use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{anime, anime_id, episodes};

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
    pub category: String,
    pub sub_or_dub: String,
    pub total_episodes: i32,
}

#[derive(Queryable, Insertable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = anime_id)]
pub struct AnimeID {
    pub anime_name: String,
}

#[derive(Queryable, Insertable, Selectable, Debug, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = episodes)]
pub struct Episode {
    pub id: String,
    pub title: String,
    pub is_filler: bool,
    pub episode_no: i32,
    pub anime_id: i32,
}
