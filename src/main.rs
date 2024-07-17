use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::error::Error;

pub mod model;
pub mod schema;

use crate::model::Anime;
use crate::schema::animes;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

fn new_anime(
    conn: &mut PgConnection,
    id: &str,
    title: &str,
    description: &str,
) -> Result<Anime, Box<dyn Error + Send + Sync>> {
    let new_anime = diesel::insert_into(animes::table)
        .values((
            animes::id.eq(id),
            animes::title.eq(title),
            animes::description.eq(description),
        ))
        .returning(Anime::as_returning())
        .get_result(conn)?;

    Ok(new_anime)
}

fn main() {
    let conn = &mut establish_connection();
    match new_anime(
        conn,
        "anime-id-90",
        "anime-name unknown gg",
        "fm3oimi 23irmi23ri2i3 m2i3mrim23imrmefm2 3mfi2m3 23m r",
    ) {
        Ok(new_anime) => println!("Inserted new anime: {:?}", new_anime),
        Err(e) => eprintln!("Error inserting anime: {}", e),
    }
}
