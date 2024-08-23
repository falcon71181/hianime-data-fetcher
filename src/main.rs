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

use model::Episode;
use operations::atoz_ops::get_last_page_no_of_atoz_list;
use operations::episode_ops::{
    add_new_episode, fetch_anime_details, load_proxies, store_anime_and_episode_data,
};
use operations::staff_ops::fetch_jikan_staff_response;
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
    println!("{:?}", fetch_jikan_staff_response(21).await.unwrap());
    // println!("{}", get_last_page_no_of_atoz_list().await.unwrap());
    // add_new_anime_with_anime_id().await;
    // store_anime_and_episode_data().await;
    // let res = fetch_anime_details("jujutsu-kaisen-2nd-season-18413".to_string()).await;
    // println!("{:?}", res);
    // let anime_list = load_all_anime_ids();
    // println!("{:?}", anime_list);
    // store_anime_and_episode_data().await;
    // let new_anime = Anime {
    //     id: 18075,
    //     title: "Overlord IV".to_string(),
    //     mal_id: 48895,
    //     al_id: 133844,
    //     japanese_title: Some("".to_string()),
    //     image: "https://cdn.noitatnemucod.net/thumbnail/300x400/100/ef1d1028c6c177587805651b78282a6.jpg".to_string(),
    //     description: "E-Rantel, the capital city of the newly established Sorcerer Kingdom, suffers from a dire shortage of goods. Once a prosperous city known for its trade, it now faces a crisis due to its caution—or even fear—of its king, Ainz Ooal Gown. To make amends, Ainz sends Albedo to the city as a diplomatic envoy.\n\nMeanwhile, the cardinals of the Slane Theocracy discuss how to retaliate against Ainz after his attack crippled the Re-Estize Kingdom's army, plotting for the Baharuth Empire to take over the Sorcerer Kingdom. However, when Emperor Jircniv Rune Farlord El Nix arranges a meeting with the Theocracy's messengers at a colosseum, he is confronted by none other than Ainz himself.\n\nWith their secret gathering now out in the open, the emperor and his guests learn that Ainz has challenged the Warrior King, the empire's greatest fighter, to a duel. With Ainz's motivations beyond his comprehension, Jircniv can do nothing but watch as humanity's future changes before his very eyes.".to_string(),
    //     type_: "TV".to_string(),
    //     sub_or_dub: "both".to_string()
    // };
    //
    // add_new_anime(new_anime, &mut connection)?;
    // load_all_anime(&mut connection);

    Ok(())
}
