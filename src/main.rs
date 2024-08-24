mod db;
mod model;
mod schema;
mod operations {
    pub mod anime_ops;
    pub mod atoz_ops;
    pub mod episode_ops;
    pub mod staff_ops;
}

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::vec;

use model::Episode;
use operations::anime_ops::load_all_anime_mal_id;
use operations::atoz_ops::get_last_page_no_of_atoz_list;
use operations::episode_ops::{
    add_new_episode, fetch_anime_details, load_proxies, store_anime_and_episode_data,
};
use operations::staff_ops::{
    fetch_jikan_staff_response, insert_into_anime_staff, insert_or_update_staff,
    store_staff_and_anime_staff, StaffResponse,
};
use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::anime_ops::{
    add_new_anime, add_new_anime_with_anime_id, load_all_anime, load_all_anime_ids,
};
use crate::db::establish_connection;
use crate::model::Anime;
use crate::operations::anime_ops;
use diesel::result::Error as DieselError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let proxies = load_proxies().await?;
    // let response = fetch_jikan_staff_response(21, &proxies).await.unwrap();
    // println!("{:?}", response);
    // let veec = response.data.iter();
    //
    // for i in veec {
    //     insert_or_update_staff(i).await;
    //     insert_into_anime_staff(i, 21).await;
    // }

    // add_new_anime_with_anime_id().await;
    // store_anime_and_episode_data().await;
    store_staff_and_anime_staff().await?;
    // println!("{:?}", load_all_anime_mal_id().unwrap().len());
    Ok(())
}
