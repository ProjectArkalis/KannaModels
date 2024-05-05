use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Episode {
    pub id: Option<String>,
    pub thumbnail: Option<String>,
    pub name: Option<String>,
    pub is_hidden: bool,
    pub is_nsfw: bool,
    pub lbry_url: String,
    pub lbry_id: Option<String>,
}
