use diesel::prelude::*;

mod db;
mod model;
mod schema;

use crate::db::establish_connection;
use crate::model::Anime;
use crate::schema::anime;

fn add_new_anime(
    new_anime: Anime,
    connection: &mut PgConnection,
) -> Result<(), diesel::result::Error> {
    diesel::insert_into(anime::table)
        .values(&new_anime)
        .execute(connection)?;

    Ok(())
}

fn main() -> Result<(), diesel::result::Error> {
    let mut connection = establish_connection();
    let new_anime = Anime {
        id: 1,
        title: String::from("Attack on Titan"),
        description: String::from("Humanity fights for survival against giants."),
    };

    add_new_anime(new_anime, &mut connection)?;

    Ok(())
}
