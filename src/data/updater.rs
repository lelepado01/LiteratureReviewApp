use crate::{data::loader::load_papers, categories::categories_data::CategoryTag};

use super::loader::{load_categories, load_categories_data};


pub fn update_categories(file_name: &str, label: &str) {
    let mut papers = load_papers();

    for paper in papers.iter_mut() {
        if paper.file_name == file_name {
            let mut found = false;
            for category in paper.categories.iter() {
                if category == &label {
                    found = true;
                }
            }
            if !found {
                paper.categories.push(label.to_string());
            } else {
                paper.categories.retain(|cat| cat != &label);
            }
        }
    }

    let mut file = std::fs::File::create("metadata/papers.ron").unwrap();
    ron::ser::to_writer(&mut file, &papers).unwrap();
}

pub fn add_category_data(category : String) {
    let mut category_data = load_categories_data();
    category_data.push(CategoryTag { label: category, color: "#ff0000".to_string() });

    let mut file = std::fs::File::create("metadata/categories.ron").unwrap();
    ron::ser::to_writer(&mut file, &category_data).unwrap();
}

pub fn delete_category_data(category : String) {
    let mut category_data = load_categories_data();
    category_data.retain(|row| row.label != category);

    let mut file = std::fs::File::create("metadata/categories.ron").unwrap();
    ron::ser::to_writer(&mut file, &category_data).unwrap();

    let mut papers = load_papers();
    for paper in papers.iter_mut() {
        paper.categories.retain(|cat| cat != &category);
    }

    let mut file = std::fs::File::create("metadata/papers.ron").unwrap();
    ron::ser::to_writer(&mut file, &papers).unwrap();
}

pub fn update_category_color(category : String, color : String) {
    let mut category_data = load_categories_data();

    for row in category_data.iter_mut() {
        if row.label == category {
            row.color = color.clone();
        }
    }

    let mut file = std::fs::File::create("metadata/categories.ron").unwrap();
    ron::ser::to_writer(&mut file, &category_data).unwrap();
}