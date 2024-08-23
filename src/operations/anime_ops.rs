// anime_ops.rs

extern crate reqwest;
extern crate serde;

use crate::db::establish_connection;
use crate::model::{Anime, AnimeID};
use crate::operations::atoz_ops::get_last_page_no_of_atoz_list;
use crate::schema::{anime, anime_id};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use dotenvy::dotenv;
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use std::boxed::Box;
use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::Formatter;
use tokio::task::JoinError;
use tokio::task::JoinHandle;

// Define CustomError for handling multiple error types
#[derive(Debug)]
pub enum CustomError {
    JoinError(JoinError),
    DieselError(DieselError),
    ReqwestError(ReqwestError),
    NoProxiesAvailable,
    FailedToFetchAfterRetries,
    Other(String),
}

// Implement `fmt::Display` for `CustomError`
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::JoinError(err) => write!(f, "Join Error: {}", err),
            CustomError::DieselError(err) => write!(f, "Diesel Error: {}", err),
            CustomError::ReqwestError(err) => write!(f, "Reqwest Error: {}", err),
            CustomError::NoProxiesAvailable => write!(f, "No proxies available"),
            CustomError::FailedToFetchAfterRetries => write!(f, "Failed to fetch after retries"),
            CustomError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

// Implement conversions from specific error types to CustomError
impl From<JoinError> for CustomError {
    fn from(err: JoinError) -> Self {
        CustomError::JoinError(err)
    }
}

impl From<DieselError> for CustomError {
    fn from(err: DieselError) -> Self {
        CustomError::DieselError(err)
    }
}

impl From<ReqwestError> for CustomError {
    fn from(err: ReqwestError) -> Self {
        CustomError::ReqwestError(err)
    }
}

// Implement `From<Box<dyn StdError>>` for `CustomError`
impl From<Box<dyn StdError>> for CustomError {
    fn from(err: Box<dyn StdError>) -> Self {
        CustomError::Other(err.to_string())
    }
}

// Implement `StdError` for `CustomError`
impl StdError for CustomError {}

// Function to add a new anime to the database
pub fn add_new_anime(new_anime: Anime) -> Result<(), DieselError> {
    let mut connection = establish_connection();
    use crate::schema::anime::dsl::*;

    // Check if the anime already exists
    let anime_exists = diesel::select(diesel::dsl::exists(anime.filter(id.eq(&new_anime.id))))
        .get_result(&mut connection)?;

    if anime_exists {
        // Update existing anime
        diesel::update(anime.find(new_anime.id))
            .set((
                title.eq(new_anime.title),
                description.eq(new_anime.description),
                mal_id.eq(new_anime.mal_id),
                al_id.eq(new_anime.al_id),
                japanese_title.eq(new_anime.japanese_title),
                synonyms.eq(new_anime.synonyms),
                image.eq(new_anime.image),
                category.eq(new_anime.category),
                rating.eq(new_anime.rating),
                quality.eq(new_anime.quality),
                duration.eq(new_anime.duration),
                premiered.eq(new_anime.premiered),
                aired.eq(new_anime.aired),
                status.eq(new_anime.status),
                mal_score.eq(new_anime.mal_score),
                studios.eq(new_anime.studios),
                producers.eq(new_anime.producers),
                genres.eq(new_anime.genres),
                sub_episodes.eq(new_anime.sub_episodes),
                dub_episodes.eq(new_anime.dub_episodes),
                total_episodes.eq(new_anime.total_episodes),
                sub_or_dub.eq(new_anime.sub_or_dub),
            ))
            .execute(&mut connection)?;
    } else {
        // Insert new anime
        diesel::insert_into(anime)
            .values(&new_anime)
            .execute(&mut connection)?;
    }

    Ok(())
}
// Function to delete an anime by its ID
pub fn delete_anime_by_id(
    anime_id: i32,
    connection: &mut PgConnection,
) -> Result<usize, DieselError> {
    let deleted_rows =
        diesel::delete(anime::table.filter(anime::id.eq(anime_id))).execute(connection)?;

    Ok(deleted_rows)
}

// Function to load all anime from the database
pub fn load_all_anime() -> Result<(), DieselError> {
    let mut connection = establish_connection();
    use crate::schema::anime::dsl::*;
    let results = anime
        .select(Anime::as_select())
        .load::<Anime>(&mut connection)?;
    println!("{:?}", results);
    Ok(())
}

// Function to load all anime_ids from the database
pub fn load_all_anime_ids() -> Result<Vec<String>, DieselError> {
    let mut connection = establish_connection();
    use crate::schema::anime_id::dsl::*;
    let results = anime_id
        .select(anime_name)
        .load::<String>(&mut connection)?;
    Ok(results)
}

// Struct for deserializing anime data from API
#[derive(Debug, Deserialize)]
struct AnimeName {
    id: String,
    name: String,
    img: String,
    episodes: Episodes,
    duration: String,
    rated: bool,
}

// Struct for deserializing episode data within AnimeName
#[derive(Debug, Deserialize)]
struct Episodes {
    eps: Option<u32>,
    sub: Option<u32>,
    dub: Option<u32>,
}

// Function to asynchronously fetch anime data from an API
// TODO: impl custom api and proxies
pub async fn fetch_data(page_no: u16) -> Result<Vec<AnimeID>, CustomError> {
    dotenv().ok();

    let atoz_list_url = env::var("ATOZLIST_URL").expect("ATOZLIST_URL must be set");
    let url = format!("{}{}", atoz_list_url, page_no);

    let response = reqwest::get(&url).await?.error_for_status()?;
    let anime_list: Vec<AnimeName> = response.json().await?;

    let anime_ids: Vec<AnimeID> = anime_list
        .iter()
        .map(|anime| AnimeID {
            anime_name: anime.id.clone(),
        })
        .collect();

    Ok(anime_ids)
}

// Function to add new anime with corresponding anime IDs
pub async fn add_new_anime_with_anime_id() -> Result<(), CustomError> {
    let mut handles: Vec<JoinHandle<Result<(), CustomError>>> = vec![];
    // TODO: use web scraping to find last page no
    let NO_OF_PAGES: u16 = get_last_page_no_of_atoz_list().await?;
    let mut count: u16 = 0;

    while count < NO_OF_PAGES {
        for i in 0..10 {
            let handle = tokio::spawn(async move {
                let page_number = count + i + 1;
                if page_number <= NO_OF_PAGES {
                    match fetch_data(page_number).await {
                        Ok(anime_ids) => {
                            for anime_id in anime_ids {
                                insert_into_anime_id(&anime_id)?;
                            }
                        }
                        Err(e) => (eprint!("{}", e)),
                    }
                }
                Ok(())
            });
            handles.push(handle);
        }
        count += 10;
    }

    // Wait for all tasks to complete and handle any errors
    for handle in handles {
        handle.await??;
    }

    println!("{}", "Anime IDs fetching Complete.");

    Ok(())
}

// Function to insert new anime ID into the anime_id table
fn insert_into_anime_id(new_anime: &AnimeID) -> Result<(), DieselError> {
    let mut connection = establish_connection();
    use crate::schema::anime_id::dsl::*;

    // Check if the anime ID already exists
    let anime_exists = diesel::select(diesel::dsl::exists(
        anime_id.filter(anime_name.eq(&new_anime.anime_name)),
    ))
    .get_result(&mut connection)?;

    if anime_exists {
        return Ok(());
    }

    diesel::insert_into(anime_id)
        .values(new_anime)
        .execute(&mut connection)?;

    Ok(())
}
