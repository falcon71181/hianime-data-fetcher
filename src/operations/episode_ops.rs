use crate::db::establish_connection;
use crate::model::{Anime, Episode};
use crate::operations::anime_ops::{add_new_anime, load_all_anime_ids};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use dotenvy::dotenv;
use rand::seq::SliceRandom;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use tokio::task::JoinHandle;
use tokio::time::Duration;

use super::anime_ops::CustomError;

// Define a struct to hold proxy data
#[derive(Debug, Clone)]
pub struct Proxy {
    pub address: String,
}

// Function to get a random proxy from the list
pub fn get_random_proxy(proxies: &[Proxy]) -> Option<Proxy> {
    proxies.choose(&mut rand::thread_rng()).cloned()
}

// Fetch proxy list from URL
pub async fn fetch_proxy_list(url: &str) -> Result<Vec<Proxy>, CustomError> {
    let client = Client::new();
    let response = client.get(url).send().await?.text().await?;
    let proxies = response
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.is_empty() {
                Some(Proxy {
                    address: line.to_string(),
                })
            } else {
                None
            }
        })
        .collect();
    Ok(proxies)
}

// Load proxies from multiple sources
pub async fn load_proxies() -> Result<Vec<Proxy>, CustomError> {
    dotenv().ok();

    let sock5_url = env::var("SOCK5_URL").expect("SOCK5_URL must be set");
    let sock4_url = env::var("SOCK4_URL").expect("SOCK4_URL must be set");
    let http_url = env::var("HTTP_URL").expect("HTTP_URL must be set");

    let (sock5_proxies, sock4_proxies, http_proxies) = tokio::try_join!(
        fetch_proxy_list(&sock5_url),
        fetch_proxy_list(&sock4_url),
        fetch_proxy_list(&http_url)
    )?;

    let mut all_proxies = Vec::new();
    all_proxies.extend(sock5_proxies);
    all_proxies.extend(sock4_proxies);
    all_proxies.extend(http_proxies);

    Ok(all_proxies)
}

// Struct for deserializing anime data from API
#[derive(Debug, Deserialize)]
pub struct AnimeDetails {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub mal_id: Option<i32>,
    pub al_id: Option<i32>,
    pub japanese_title: Option<String>,
    pub synonyms: Option<String>,
    pub image: Option<String>,
    pub category: Option<String>,
    pub rating: Option<String>,
    pub quality: Option<String>,
    pub duration: Option<String>,
    pub premiered: Option<String>,
    pub aired: Option<String>,
    pub status: Option<String>,
    pub mal_score: Option<String>,
    pub studios: Option<String>,
    pub producers: Option<String>,
    pub genres: Option<String>,
    pub sub_episodes: Option<i32>,
    pub dub_episodes: Option<i32>,
    pub total_episodes: Option<i32>,
    pub sub_or_dub: Option<String>,
    pub episodes: Option<Vec<EpisodeDetails>>,
}

// Struct for deserializing episode data from API
#[derive(Debug, Deserialize)]
pub struct EpisodeDetails {
    pub id: Option<String>,
    pub title: Option<String>,
    pub is_filler: Option<bool>,
    pub episode_no: Option<i32>,
}

impl Default for AnimeDetails {
    fn default() -> Self {
        AnimeDetails {
            id: 0,
            title: Some(String::from("")),
            description: Some(String::from("")),
            mal_id: Some(0),
            al_id: Some(0),
            japanese_title: Some(String::from("")),
            synonyms: Some(String::from("")),
            image: Some(String::from("")),
            category: Some(String::from("")),
            rating: Some(String::from("")),
            quality: Some(String::from("")),
            duration: Some(String::from("")),
            premiered: Some(String::from("")),
            aired: Some(String::from("")),
            status: Some(String::from("")),
            mal_score: Some(String::from("n/a")),
            studios: Some(String::from("")),
            producers: Some(String::from("")),
            genres: Some(String::from("")),
            sub_episodes: Some(0),
            dub_episodes: Some(0),
            total_episodes: Some(0),
            sub_or_dub: Some(String::from("")),
            episodes: Some(vec![]),
        }
    }
}

impl Default for EpisodeDetails {
    fn default() -> Self {
        EpisodeDetails {
            id: Some(String::from("")),
            title: Some(String::from("")),
            is_filler: Some(false),
            episode_no: Some(0),
        }
    }
}

// Add new episode to the database
pub fn add_new_episode(new_episode: Episode) -> Result<(), DieselError> {
    let mut connection = establish_connection();
    use crate::schema::episodes::dsl::*;

    // Check if the episode already exists
    let episode_exists =
        diesel::select(diesel::dsl::exists(episodes.filter(id.eq(&new_episode.id))))
            .get_result(&mut connection)?;

    if episode_exists {
        // Update existing episode
        diesel::update(episodes.filter(id.eq(&new_episode.id)))
            .set((
                title.eq(&new_episode.title),
                is_filler.eq(new_episode.is_filler),
                episode_no.eq(new_episode.episode_no),
                anime_id.eq(new_episode.anime_id),
            ))
            .execute(&mut connection)?;
    } else {
        // Insert new episode
        diesel::insert_into(episodes)
            .values(&new_episode)
            .execute(&mut connection)?;
    }

    Ok(())
}

// Function to asynchronously fetch anime data from an API
pub async fn fetch_anime_details(
    anime_id: String,
    proxies: &[Proxy],
) -> Result<AnimeDetails, CustomError> {
    let mut attempts = 0;
    let max_attempts = 5;
    dotenv().ok();
    let anime_fetcher_url = env::var("ANIME_FETCHER_URL").expect("ANIME_FETCHER_URL must be set.");

    while attempts < max_attempts {
        if let Some(proxy) = get_random_proxy(proxies) {
            let client = Client::builder()
                .proxy(reqwest::Proxy::http(&proxy.address)?)
                .timeout(Duration::from_secs(5))
                .build()?;

            let url = format!("{}/{}", anime_fetcher_url, anime_id);
            let response = client.get(&url).send().await;

            match response {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let anime_data = resp.json().await?;
                        return Ok(anime_data);
                    } else {
                        eprintln!("Failed with status: {:?}", resp.status());
                    }
                }
                Err(e) => eprintln!("Failed to fetch with proxy: {:?}. Error: {:?}", proxy, e),
            }
        } else {
            return Err(CustomError::NoProxiesAvailable);
        }

        attempts += 1;
        // tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Err(CustomError::FailedToFetchAfterRetries)
}

// Store anime and episode data
pub async fn store_anime_and_episode_data() -> Result<(), CustomError> {
    let anime_list = load_all_anime_ids().map_err(|e| CustomError::from(e))?;
    let proxies = load_proxies().await?;

    let mut handles: Vec<JoinHandle<Result<(), CustomError>>> = vec![];
    let no_of_animes: usize = anime_list.len();
    let chunk_size: usize = 100;

    let mut count: usize = 0;
    while count < no_of_animes {
        let end = (count + chunk_size).min(no_of_animes);
        let chunk: Vec<_> = anime_list[count..end].to_vec();

        let proxies = proxies.clone();

        let handle = tokio::spawn(async move {
            for anime in chunk {
                match fetch_anime_details(anime, &proxies).await {
                    Ok(mut anime_data) => {
                        anime_data.title = anime_data.title.or(Some(String::from("Unknown Title")));
                        anime_data.description = anime_data
                            .description
                            .or(Some(String::from("No description available")));

                        let anime_detail = Anime {
                            id: anime_data.id,
                            title: anime_data.title.unwrap_or_default(),
                            description: anime_data.description.unwrap_or_default(),
                            mal_id: anime_data.mal_id.unwrap_or_default(),
                            al_id: anime_data.al_id.unwrap_or_default(),
                            japanese_title: Some(anime_data.japanese_title.unwrap_or_default()),
                            synonyms: Some(anime_data.synonyms.unwrap_or_default()),
                            image: anime_data.image.unwrap_or_default(),
                            category: anime_data.category.unwrap_or_default(),
                            rating: anime_data.rating.unwrap_or_default(),
                            quality: anime_data.quality.unwrap_or_default(),
                            duration: anime_data.duration.unwrap_or_default(),
                            premiered: anime_data.premiered.unwrap_or_default(),
                            aired: anime_data.aired.unwrap_or_default(),
                            status: anime_data.status.unwrap_or_default(),
                            mal_score: anime_data.mal_score.unwrap_or_default(),
                            studios: anime_data.studios.unwrap_or_default(),
                            producers: anime_data.producers.unwrap_or_default(),
                            genres: anime_data.genres.unwrap_or_default(),
                            sub_episodes: anime_data.sub_episodes.unwrap_or_default(),
                            dub_episodes: anime_data.dub_episodes.unwrap_or_default(),
                            total_episodes: anime_data.total_episodes.unwrap_or_default(),
                            sub_or_dub: anime_data.sub_or_dub.unwrap_or_default(),
                        };
                        add_new_anime(anime_detail)?;
                        println!("{}", anime_data.id);

                        if let Some(episodes) = anime_data.episodes {
                            for episode_data in episodes {
                                let episode_detail = Episode {
                                    id: episode_data.id.unwrap_or_default(),
                                    title: episode_data.title.unwrap_or_default(),
                                    is_filler: episode_data.is_filler.unwrap_or_default(),
                                    episode_no: episode_data.episode_no.unwrap_or_default(),
                                    anime_id: anime_data.id,
                                };
                                add_new_episode(episode_detail)?;
                            }
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
        if let Err(e) = handle.await? {
            eprintln!("Task failed: {:?}", e);
        }
    }

    println!("{}", "Anime and Episode Data fetching Complete.");

    Ok(())
}
