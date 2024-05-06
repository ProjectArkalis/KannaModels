use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Episode {
    pub id: String,
    pub source: u32,
    pub name: String,
    pub hidden: bool,
    pub nsfw: bool,
    pub lbry_id: String,
    pub file_url: String,
}
