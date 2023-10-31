
use crate::categories::categories_data::CategoryTag;
use crate::data::Paper;

pub fn to_categories_file(data : &Vec<CategoryTag>) {
    let mut file = std::fs::File::create("metadata/categories.ron").unwrap();
    ron::ser::to_writer(&mut file, &data).unwrap();
}

pub fn to_papers_file(data : &Vec<Paper>) {
    let mut file = std::fs::File::create("metadata/papers.ron").unwrap();
    ron::ser::to_writer(&mut file, &data).unwrap();
}

