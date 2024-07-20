mod db;
mod model;
mod schema;
mod operations {
    pub mod anime_ops;
}

use crate::anime_ops::{add_new_anime, load_all_anime};
use crate::db::establish_connection;
use crate::model::Anime;
use crate::operations::anime_ops;

fn main() -> Result<(), diesel::result::Error> {
    let mut connection = establish_connection();
    let new_anime = Anime {
        id: 1,
        title: String::from("Attack on Titan"),
        description: String::from("Humanity fights for survival against giants."),
    };

    add_new_anime(new_anime, &mut connection)?;
    load_all_anime(&mut connection);

    Ok(())
}
