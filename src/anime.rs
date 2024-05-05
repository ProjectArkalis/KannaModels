use nestify::nest;
use serde::{Deserialize, Serialize};

//meio cursed*
nest! {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]*
    pub struct Anime {
        pub id: Option<u32>,
        pub titles: Vec<pub enum Title {
            //n sei se isso vai ficar assim :)
            Romaji(pub struct TitleProps {
                pub title: String,
                pub main: bool,
            }),
            English(TitleProps),
            Portuguese(TitleProps),
            Native(TitleProps),
        }>,
        pub synopsis: String,
        pub images: pub struct AnimeImages {
            pub thumbnail_id: Option<String>,
            pub banner_id: Option<String>,
        },
        pub is_hidden: bool,
        pub is_nsfw: bool,
        pub created_by: String,
        pub created_at: u64,
        pub release_date: u64,
        pub lists: Vec<
            #[derive(Clone, Copy, Eq)]
            pub enum AnimeList {
                MyAnimeList(i64),
                Anilist(i64)
            }
        >,
        //isso vai mudar
        pub genre: u64
    }
}
