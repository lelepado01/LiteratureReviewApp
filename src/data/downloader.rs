

use std::fs::File;
use std::io::Write;

use crate::data::Paper;
use crate::export::export_data::CitationData;
use crate::scholar::scholar::{init_client, ScholarArgs};

pub enum DownloaderResult<T> {
    Ok(T), 
    Err(DownloaderError), 
}

impl<T> DownloaderResult<T> {
    pub fn is_ok(&self) -> bool {
        match self {
            DownloaderResult::Ok(_) => true,
            DownloaderResult::Err(_) => false,
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            DownloaderResult::Ok(t) => t,
            DownloaderResult::Err(e) => panic!("called `DownloaderResult::unwrap()` on an `Err` value: {:?}", e),
        }
    }
}

#[derive(Debug)]
pub enum DownloaderError {
    Download,
    Status,
    ExtractBytes,
    Scholar, 
    PaperNotFound,
}

pub fn download_paper(link : String, title : String, authors: String) -> DownloaderResult<Paper> {

    let saving_file_name = (title.clone() + " " + authors.clone().as_str() + ".pdf").replace(' ', "_");

    // let file_name = link.clone().split('/').last().unwrap().to_owned();
    let res = reqwest::get(link); 

    if let Ok(resp) = pollster::block_on(res){
        if resp.status() != 200 {
            // println!("Error downloading paper: {}, status: {}", file_name, resp.status());
            return DownloaderResult::Err(DownloaderError::Status); 
        }
        if let Ok(bytes) = pollster::block_on(resp.bytes()) {
            let mut file = File::create("papers/".to_owned() + saving_file_name.as_str()).unwrap();
            file.write_all(&bytes).unwrap();
        } else {
            // println!("Error extracting bytes: {}", file_name);
            return DownloaderResult::Err(DownloaderError::ExtractBytes); 
        }
    } else {
        // println!("Error downloading paper: {}", file_name);
        return DownloaderResult::Err(DownloaderError::Download);
    }
    
    DownloaderResult::Ok(Paper {
        file_name: saving_file_name,  
        title,
        authors: authors.clone(),
        categories: vec![]
    })
}

pub fn download_paper_citation(paper_name : String) -> DownloaderResult<CitationData> {

        let s = ScholarArgs {
            query: paper_name.as_str(),
            cite_id: None,
            from_year: None,
            to_year: None,
            sort_by: Some(0),
            cluster_id: None,
            lang: Some("en"),
            lang_limit: None,
            limit: Some(3),
            offset: Some(0),
            adult_filtering: None,
            include_similar_results: None,
            include_citations: None,
        };
        
        let client = init_client();
        match pollster::block_on(client.scrape_scholar(&s)) {
            Ok(result) => {
                if !result.is_empty() {
                    DownloaderResult::Ok(CitationData {
                        author: result[0].author.clone(),
                        title: result[0].title.clone(),
                        abstract_data: result[0].abs.clone(),
                        link: result[0].link.clone(),
                    })
                } else {
                    DownloaderResult::Err(DownloaderError::PaperNotFound)
                }
            }
            Err(_e) => {
                DownloaderResult::Err(DownloaderError::Scholar)
            }
        }
}
