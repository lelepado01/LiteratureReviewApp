
pub mod loader; 
pub mod updater;
pub mod downloader;
pub mod searcher;
mod file_helper;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Paper {
    pub file_name: String,
    pub title: String,
    pub authors: String,
    pub categories: Vec<String>,
}