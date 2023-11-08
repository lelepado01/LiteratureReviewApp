use futures::stream::FuturesUnordered;

use crate::global_search::global_search_results::GlobalSearchResult;
use super::loader::{LoaderResult, load_papers, load_pdf_content};


pub async fn search_in_all_files_async(query : &str) -> FuturesUnordered<GlobalSearchResult> {
    let all_pdfs = load_papers();
    if let LoaderResult::Ok(all_pdfs) = all_pdfs {
        all_pdfs.iter().filter_map(|pdf| {
            let paper_data = load_pdf_content(&pdf.file_name);
            if let LoaderResult::Ok(paper_data) = paper_data {
                paper_data.find(query).map(|index| GlobalSearchResult {
                    file_name: pdf.file_name.clone(),
                    file_content: paper_data[index..index+100].to_string(),
                })
            } else {
                None
            }
        })
        .collect::<FuturesUnordered<_>>()
    } else {
        FuturesUnordered::new()
    }
}

pub fn search_in_all_files(query : &str) -> Vec<GlobalSearchResult> {
    let mut results = vec![];
    let all_pdfs = load_papers();

    if let LoaderResult::Ok(all_pdfs) = all_pdfs {
        for pdf in all_pdfs {
            let paper_data = load_pdf_content(&pdf.file_name);
            if let LoaderResult::Ok(paper_data) = paper_data {
                
                if let Some(index) = paper_data.find(query) {
                    results.push(GlobalSearchResult {
                        file_name: pdf.file_name, 
                        file_content: paper_data[index..index+100].to_string(),
                    })

                }
            }
        }
    } 

    results
}
