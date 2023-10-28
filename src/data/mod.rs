
pub mod loader; 
pub mod updater;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Paper {
    pub file_name: String,
    pub title: String,
    pub categories: Vec<String>,
}