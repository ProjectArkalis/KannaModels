use crate::{genres::Genres, season::Season};
use nestify::nest;
use serde::{Deserialize, Serialize};

nest! {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]*
    pub struct Anime {
        pub id: Option<u32>,
        pub titles: pub struct Titles {
            pub english: String,
            pub romaji: String,
            pub native: String
        },
        pub synopsis: String,
        pub genres: Vec<Genres>,
        pub nsfw: bool,
        pub hidden: bool,
        //todo
        pub release_date: String,
        pub lists: pub struct AnimeLists {
            pub anilist: u32,
            pub mal: u32
        },
        pub seasons: Vec<Season>
    }
}
