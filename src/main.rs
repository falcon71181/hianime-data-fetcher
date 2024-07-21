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
        id: 18075,
        title: "Overlord IV".to_string(),
        mal_id: 48895,
        al_id: 133844,
        japanese_title: Some("".to_string()),
        image: "https://cdn.noitatnemucod.net/thumbnail/300x400/100/ef1d1028cf6c177587805651b78282a6.jpg".to_string(),
        description: "E-Rantel, the capital city of the newly established Sorcerer Kingdom, suffers from a dire shortage of goods. Once a prosperous city known for its trade, it now faces a crisis due to its caution—or even fear—of its king, Ainz Ooal Gown. To make amends, Ainz sends Albedo to the city as a diplomatic envoy.\n\nMeanwhile, the cardinals of the Slane Theocracy discuss how to retaliate against Ainz after his attack crippled the Re-Estize Kingdom's army, plotting for the Baharuth Empire to take over the Sorcerer Kingdom. However, when Emperor Jircniv Rune Farlord El Nix arranges a meeting with the Theocracy's messengers at a colosseum, he is confronted by none other than Ainz himself.\n\nWith their secret gathering now out in the open, the emperor and his guests learn that Ainz has challenged the Warrior King, the empire's greatest fighter, to a duel. With Ainz's motivations beyond his comprehension, Jircniv can do nothing but watch as humanity's future changes before his very eyes.".to_string(),
        type_: "TV".to_string(),
        sub_or_dub: "both".to_string()
    };

    add_new_anime(new_anime, &mut connection)?;
    load_all_anime(&mut connection);

    Ok(())
}
