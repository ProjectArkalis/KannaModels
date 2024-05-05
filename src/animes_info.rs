use crate::{anime::Anime, season::Season};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AnimesInfo {
    pub anime: Anime,
    pub seasons: Vec<Season>,
}
