

use crate::{data::loader::load_papers, categories::categories_data::CategoryTag, general_memos::general_memo_data::Memo};
use super::{loader::{load_categories_data, LoaderResult, load_paper_files}, file_helper::{to_papers_file, to_categories_file, to_memos_file}};
use crate::data::Paper;

pub fn add_paper_data(paper_data: Paper) {
    let papers_res = load_papers();
    if let LoaderResult::Ok(mut papers) = papers_res {
        papers.push(paper_data);
        to_papers_file(&papers);
    }     
}

pub fn update_paper_data() {
    let papers = load_papers();
    let paper_files = load_paper_files(); 

    if let (LoaderResult::Ok(papers), LoaderResult::Ok(paper_files)) = (papers, paper_files) {
        for paper_file in paper_files.iter() {
            let mut found = false;
            for paper_data in papers.iter() {
                if paper_file == &paper_data.file_name {
                    found = true;
                    break;
                }
            }
            if !found {
                add_paper_data(Paper{
                    file_name: paper_file.clone(),
                    title: paper_file.clone(),
                    authors: "".to_string(),
                    categories: vec![],
                }); 
            } 
        }
    }
}

pub fn update_categories(file_name: &str, label: &str) {
    let papers = load_papers();

    if let LoaderResult::Ok(mut papers) = papers {
        for paper in papers.iter_mut() {
            if paper.file_name == file_name {
                let mut found = false;
                for category in paper.categories.iter() {
                    if category == label {
                        found = true;
                        break;
                    }
                }
                if !found {
                    paper.categories.push(label.to_string());
                } else {
                    paper.categories.retain(|cat| cat != label);
                }
            }
        }
    
        to_papers_file(&papers);
    }
    
}

pub fn add_category_data(category : String) {
    let category_data = load_categories_data();
    
    if let LoaderResult::Ok(mut category_data) = category_data {
        category_data.push(CategoryTag { label: category, color: "#000000".to_string() });
        to_categories_file(&category_data);
    }

}

pub fn delete_category_data(category : String) {
    let category_data = load_categories_data();
    
    if let LoaderResult::Ok(mut category_data) = category_data {
        category_data.retain(|row| row.label != category);

        to_categories_file(&category_data);

        let papers = load_papers();
        if let LoaderResult::Ok(mut papers) = papers {
            for paper in papers.iter_mut() {
                paper.categories.retain(|cat| cat != &category);
            }

            to_papers_file(&papers);
        }        
    }
}

pub fn update_category_color(category : String, color : String) {
    let category_data = load_categories_data();

    if let LoaderResult::Ok(mut category_data) = category_data {
        for row in category_data.iter_mut() {
            if row.label == category {
                row.color = color.clone();
            }
        }
    
        to_categories_file(&category_data);
    }
}


pub fn update_memo_data(memos : Vec<Memo>) {
    to_memos_file(&memos);
}