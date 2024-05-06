use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Genres {
    Unknown,
    Action,
    Comedy,
    Horror,
    Sports,
    Adventure,
    Drama,
    Mystery,
    Supernatural,
    AvantGrade,
    Fantasy,
    Romance,
    Suspense,
    AwardWinning,
    GirlsLove,
    SciFi,
    BoysLove,
    Gourmet,
    SliceOfLife,
    Ecchi,
    Erotica,
    Hentai,
}
