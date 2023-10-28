
use std::{fs::File, io::Write};

use super::export_data::ExportData;

use gscholar::scholar::{init_client, ScholarArgs}; 

pub fn export_to_bib(export_data : ExportData) {

    let mut data : Vec<String> = vec![]; 
    for cit in  export_data.citation_data.iter() {
        let title = cit.title.clone().as_str();
        let s = ScholarArgs {
            query: "Attn",//title,
            cite_id: None,
            from_year: Some(2018),
            to_year: Some(2021),
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


    let mut file = File::create("bibliography.bib").unwrap();
    file.write_all(data.join("\n").as_bytes()).unwrap();
    file.sync_all().unwrap();
    file.flush().unwrap();
    println!("Done");

}

pub fn export_to_text(export_data : ExportData){

}