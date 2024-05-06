use crate::episode::Episode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Season {
    pub id: Option<u32>,
    pub title: String,
    pub episodes: HashMap<u32, Vec<Episode>>,
}
