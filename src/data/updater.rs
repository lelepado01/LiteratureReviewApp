

use crate::{data::loader::load_papers, categories::categories_data::CategoryTag};
use super::{loader::load_categories_data, file_helper::{to_papers_file, to_categories_file}};
use crate::data::Paper;

pub fn add_paper_data(paper_data: Paper) {
    let mut papers = load_papers();
    papers.push(paper_data);
    to_papers_file(&papers);
}


pub fn update_categories(file_name: &str, label: &str) {
    let mut papers = load_papers();

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

pub fn add_category_data(category : String) {
    let mut category_data = load_categories_data();
    category_data.push(CategoryTag { label: category, color: "#ff0000".to_string() });

    to_categories_file(&category_data);
}

pub fn delete_category_data(category : String) {
    let mut category_data = load_categories_data();
    category_data.retain(|row| row.label != category);

    to_categories_file(&category_data);

    let mut papers = load_papers();
    for paper in papers.iter_mut() {
        paper.categories.retain(|cat| cat != &category);
    }

    to_papers_file(&papers);
}

pub fn update_category_color(category : String, color : String) {
    let mut category_data = load_categories_data();

    for row in category_data.iter_mut() {
        if row.label == category {
            row.color = color.clone();
        }
    }

    to_categories_file(&category_data);
}