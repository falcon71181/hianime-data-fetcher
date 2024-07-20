// anime_ops.rs

use crate::model::Anime;
use crate::schema::anime;
use diesel::prelude::*;
use diesel::result::Error;

pub fn add_new_anime(new_anime: Anime, connection: &mut PgConnection) -> Result<(), Error> {
    diesel::insert_into(anime::table)
        .values(&new_anime)
        .execute(connection)?;

    Ok(())
}

pub fn delete_anime_by_id(anime_id: i32, connection: &mut PgConnection) -> Result<usize, Error> {
    let deleted_rows =
        diesel::delete(anime::table.filter(anime::id.eq(anime_id))).execute(connection)?;

    Ok(deleted_rows)
}

pub fn load_all_anime(connection: &mut PgConnection) -> Result<(), diesel::result::Error> {
    use crate::schema::anime::dsl::*;
    let results = anime.select(Anime::as_select()).load(connection)?;
    println!("{:?}", results);
    Ok(())
}
