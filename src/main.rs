mod db;
mod model;
mod schema;
mod operations {
    pub mod anime_ops;
    pub mod episode_ops;
}

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use model::Episode;
use operations::episode_ops::{
    add_new_episode, fetch_anime_details, load_proxies, store_anime_and_episode_data,
};
use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::anime_ops::{
    add_new_anime, add_new_anime_with_anime_id, load_all_anime, load_all_anime_ids,
};
use crate::db::establish_connection;
use crate::model::Anime;
use crate::operations::anime_ops;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    add_new_anime_with_anime_id().await;
    store_anime_and_episode_data().await;

    Ok(())
}
