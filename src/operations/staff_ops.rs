use std::env;

use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    db::establish_connection,
    model::{AnimeStaff, Staff},
    schema::anime_staff::dsl::{
        anime_id as anime_staff_anime_id, positions as anime_staff_positions,
        staff_id as anime_staff_staff_id,
    },
    schema::staff::dsl::{mal_id as staff_mal_id, positions as staff_positions},
};

use super::anime_ops::CustomError;

#[derive(Debug, Serialize, Deserialize)]
pub struct StaffResponse {
    pub data: Vec<PersonData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonData {
    person: Person,
    positions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    mal_id: i32,
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

// Refactored function for inserting or updating staff
pub fn insert_or_update_staff(staff_data: &PersonData) -> Result<(), CustomError> {
    let mut connection = establish_connection();

    // Check if the staff already exists
    let staff_exists = diesel::select(diesel::dsl::exists(
        crate::schema::staff::table.filter(staff_mal_id.eq(&staff_data.person.mal_id)),
    ))
    .get_result::<bool>(&mut connection)?;

    if staff_exists {
        // Get existing positions
        let mut existing_positions: Vec<Option<String>> = crate::schema::staff::table
            .filter(staff_mal_id.eq(&staff_data.person.mal_id))
            .select(staff_positions)
            .get_result::<Vec<Option<String>>>(&mut connection)?;

        // Merge new positions with existing positions
        let new_positions = convert_vec_string_to_vec_option_string(staff_data.positions.clone());
        for position in new_positions {
            if !existing_positions.contains(&position) {
                existing_positions.push(position);
            }
        }

        // Update the staff record with new positions
        diesel::update(
            crate::schema::staff::table.filter(staff_mal_id.eq(&staff_data.person.mal_id)),
        )
        .set(staff_positions.eq(existing_positions))
        .execute(&mut connection)?;
    } else {
        // Insert into staff if not exists
        let new_staff = Staff {
            mal_id: staff_data.person.mal_id,
            name: staff_data.person.name.clone(),
            mal_url: staff_data.person.url.clone(),
            image: staff_data.person.images.jpg.image_url.clone(),
            positions: convert_vec_string_to_vec_option_string(staff_data.positions.clone()),
        };

        diesel::insert_into(crate::schema::staff::table)
            .values(&new_staff)
            .execute(&mut connection)?;
    }

    Ok(())
}

// Refactored function for inserting into anime_staff
pub fn insert_into_anime_staff(
    staff_data: &PersonData,
    anime_table_id: i32,
) -> Result<(), CustomError> {
    let mut connection = establish_connection();

    // Check if the anime_staff relationship already exists
    let anime_staff_exists = diesel::select(diesel::dsl::exists(
        crate::schema::anime_staff::table.filter(
            anime_staff_staff_id
                .eq(&staff_data.person.mal_id)
                .and(anime_staff_anime_id.eq(&anime_table_id)),
        ),
    ))
    .get_result::<bool>(&mut connection)?;

    if !anime_staff_exists {
        // Insert into anime_staff if not exists
        let new_anime_staff = AnimeStaff {
            anime_id: anime_table_id,
            staff_id: staff_data.person.mal_id,
            positions: convert_vec_string_to_vec_option_string(staff_data.positions.clone()),
        };

        diesel::insert_into(crate::schema::anime_staff::table)
            .values(&new_anime_staff)
            .execute(&mut connection)?;
    }

    Ok(())
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

pub fn convert_vec_string_to_vec_option_string(strings: Vec<String>) -> Vec<Option<String>> {
    strings.into_iter().map(|s| Some(s)).collect()
}
