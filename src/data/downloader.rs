

use std::fs::File;
use std::io::Write;

use crate::data::Paper;

pub fn download_paper(link : String, title : String, authors: String) -> Paper {

    let saving_file_name = (title.clone() + " " + authors.clone().as_str() + ".pdf").replace(' ', "_");

    println!("Downloading paper: {:?}", link); 
    let file_name = link.clone().split('/').last().unwrap().to_owned();
    let res = reqwest::get(link); 

    if let Ok(resp) = pollster::block_on(res){
        if resp.status() != 200 {
            println!("Error downloading paper: {}, status: {}", file_name, resp.status());
        }
        if let Ok(bytes) = pollster::block_on(resp.bytes()) {
            let mut file = File::create("papers/".to_owned() + saving_file_name.as_str()).unwrap();
            file.write_all(&bytes).unwrap();
        } else {
            println!("Error extracting bytes: {}", file_name);
        }
    } else {
        println!("Error downloading paper: {}", file_name);
    }
    
    Paper {
        file_name: saving_file_name,  
        title,
        authors: authors.clone(),
        categories: vec![]
    }
}