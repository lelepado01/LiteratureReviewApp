
use std::{fs::File, io::Write};

use super::export_data::ExportData;

use crate::scholar::scholar::ScholarArgs;
use crate::scholar::scholar::init_client; 

pub fn export_to_bib(export_data : ExportData) {

    let mut data : Vec<String> = vec![]; 
    for (i, cit) in export_data.citation_data.read().iter().enumerate() {
        let tit = ""; //&export_data.citation_data[i].title; 
        let s = ScholarArgs {
            query: tit,
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
        let file_data = match pollster::block_on(client.scrape_scholar(&s)) {
            Ok(result) => result[0].author.clone(),
            Err(_e) => "help me pliz :(".to_string()
        };

        data.push(file_data);
    }


    let mut file = File::create("exports/bibliography.bib").unwrap();
    file.write_all(data.join("\n").as_bytes()).unwrap();
    file.sync_all().unwrap();
    file.flush().unwrap();
    println!("Done");

}

pub fn export_to_text(export_data : ExportData){

}