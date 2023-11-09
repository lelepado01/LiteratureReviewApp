
use crate::categories::categories_data::CategoryTag;
use crate::data::Paper;
use crate::general_memos::general_memo_data::Memo;

pub fn to_categories_file(data : &Vec<CategoryTag>) {
    let mut file = std::fs::File::create("metadata/categories.ron").unwrap();
    ron::ser::to_writer(&mut file, &data).unwrap();
}

pub fn to_papers_file(data : &Vec<Paper>) {
    let mut file = std::fs::File::create("metadata/papers.ron").unwrap();
    ron::ser::to_writer(&mut file, &data).unwrap();
}

pub fn to_memos_file(data : &Vec<Memo>) {
    let mut file = std::fs::File::create("metadata/memos.ron").unwrap();
    ron::ser::to_writer(&mut file, &data).unwrap();
}