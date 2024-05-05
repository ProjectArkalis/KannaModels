use crate::source::Source;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Season {
    pub id: Option<u32>,
    pub title: String,
    pub tumbnail: Option<String>,
    pub sources: Vec<Source>,
}
