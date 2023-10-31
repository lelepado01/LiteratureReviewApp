
pub mod loader; 
pub mod updater;
mod file_helper;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Paper {
    pub file_name: String,
    pub title: String,
    pub categories: Vec<String>,
}