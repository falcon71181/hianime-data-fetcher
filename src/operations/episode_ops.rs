use crate::db::establish_connection;
use crate::model::{Anime, Episode};
use crate::schema::{anime, episodes};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use serde::Deserialize;
use tokio::task::JoinHandle;
use tokio::time::Duration;

use super::anime_ops::{
    add_new_anime, add_new_anime_with_anime_id, load_all_anime_ids, CustomError,
};

// Struct for deserializing anime data from API
#[derive(Debug, Deserialize)]
pub struct AnimeDetails {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub mal_id: i32,
    pub al_id: i32,
    pub japanese_title: String,
    pub synonyms: String,
    pub image: String,
    pub category: String,
    pub rating: String,
    pub quality: String,
    pub duration: String,
    pub premiered: String,
    pub aired: String,
    pub status: String,
    pub mal_score: String,
    pub studios: String,
    pub producers: String,
    pub genres: String,
    pub sub_episodes: i32,
    pub dub_episodes: i32,
    pub total_episodes: i32,
    pub sub_or_dub: String,
    pub episodes: Vec<EpisodeDetails>,
}

// Struct for deserializing episode data from API
#[derive(Debug, Deserialize)]
pub struct EpisodeDetails {
    pub id: String,
    pub title: String,
    pub is_filler: bool,
    pub episode_no: i32,
}

pub fn add_new_episode(new_episode: Episode) -> Result<(), DieselError> {
    let mut connection = establish_connection();
    use crate::schema::episodes::dsl::*;

    // Check if the episode already exists
    let anime_exists = diesel::select(diesel::dsl::exists(episodes.filter(id.eq(&new_episode.id))))
        .get_result(&mut connection)?;

    if anime_exists {
        return Ok(());
    }

    diesel::insert_into(episodes)
        .values(&new_episode)
        .execute(&mut connection)?;

    Ok(())
}

// Function to asynchronously fetch anime data from an API
pub async fn fetch_anime_details(anime: String) -> Result<AnimeDetails, CustomError> {
    let url = format!("http://localhost:3001/anime/{}", anime);

    let response = reqwest::get(&url).await?.error_for_status()?;
    let anime_detail: AnimeDetails = response.json().await?;

    Ok(anime_detail)
}

pub async fn store_anime_and_episode_data() -> Result<(), CustomError> {
    // add_new_anime_with_anime_id().await?;
    let anime_list = load_all_anime_ids().unwrap();

    let mut handles: Vec<JoinHandle<Result<(), CustomError>>> = vec![];
    let no_of_animes: usize = anime_list.len();
    let chunk_size: usize = 20;

    let mut count: usize = 0;
    while count < no_of_animes {
        let end = (count + chunk_size).min(no_of_animes);
        let chunk: Vec<_> = anime_list[count..end].to_vec();

        let handle = tokio::spawn(async move {
            for anime in chunk {
                match fetch_anime_details(anime).await {
                    Ok(anime_data) => {
                        let anime_detail = Anime {
                            id: anime_data.id,
                            title: anime_data.title,
                            description: anime_data.description,
                            mal_id: anime_data.mal_id,
                            al_id: anime_data.al_id,
                            japanese_title: Some(anime_data.japanese_title),
                            synonyms: Some(anime_data.synonyms),
                            image: anime_data.image,
                            category: anime_data.category,
                            rating: anime_data.rating,
                            quality: anime_data.quality,
                            duration: anime_data.duration,
                            premiered: anime_data.premiered,
                            aired: anime_data.aired,
                            status: anime_data.status,
                            mal_score: anime_data.mal_score,
                            studios: anime_data.studios,
                            producers: anime_data.producers,
                            genres: anime_data.genres,
                            sub_episodes: anime_data.sub_episodes,
                            dub_episodes: anime_data.dub_episodes,
                            total_episodes: anime_data.total_episodes,
                            sub_or_dub: anime_data.sub_or_dub,
                        };
                        add_new_anime(anime_detail)?;

                        for episode_data in anime_data.episodes {
                            let episode_detail = Episode {
                                id: episode_data.id,
                                title: episode_data.title,
                                is_filler: episode_data.is_filler,
                                episode_no: episode_data.episode_no,
                                anime_id: anime_data.id,
                            };
                            add_new_episode(episode_detail)?;
                        }
                    }
                    Err(e) => eprintln!("Failed to fetch anime details: {:?}", e),
                }
            }
            Ok(())
        });

        handles.push(handle);
        count += chunk_size;
    }

    // Wait for all tasks to complete and handle any errors
    for handle in handles {
        tokio::time::sleep(Duration::from_secs(1)).await;
        if let Err(e) = handle.await? {
            eprintln!("Task failed: {:?}", e);
        }
    }

    Ok(())
}
