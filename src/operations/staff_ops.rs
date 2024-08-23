use std::env;

use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::anime_ops::CustomError;

#[derive(Debug, Serialize, Deserialize)]
pub struct StaffResponse {
    data: Vec<PersonData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersonData {
    person: Person,
    positions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    mal_id: u32,
    url: String,
    images: Images,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Images {
    jpg: JpgImage,
}

#[derive(Debug, Serialize, Deserialize)]
struct JpgImage {
    image_url: String,
}

pub async fn fetch_jikan_staff_response(anime_mal_id: u16) -> Result<StaffResponse, CustomError> {
    dotenv().ok();
    let jikan_api_url = env::var("JIKAN_API_URL").expect("JIKAN_API_URL must be set.");

    let client = Client::new();
    let staff_url = format!("{}/anime/{}/staff", jikan_api_url, anime_mal_id);
    let response = client.get(staff_url).send().await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let staff_data = res.json().await?;
                return Ok(staff_data);
            } else {
                eprintln!("Failed with status: {:?}", res.status());
            }
        }
        Err(_e) => eprintln!("Failed to fetch staff data for {}.", anime_mal_id),
    }

    Err(CustomError::FailedToFetchAfterRetries)
}
