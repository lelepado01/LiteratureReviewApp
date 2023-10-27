
use crate::tables::categories::CategoriesTableRow;

pub fn load_categories() -> Vec<CategoriesTableRow> {
    let file = std::fs::File::open("metadata/categories.ron").unwrap();
    let categories: Vec<CategoriesTableRow> = ron::de::from_reader(file).unwrap();

    categories
}

pub fn load_unique_categories() -> Vec<String> {
    let file = std::fs::File::open("metadata/categories.ron").unwrap();
    let categories: Vec<CategoriesTableRow> = ron::de::from_reader(file).unwrap();

    let mut result = Vec::new();

    for row in categories.iter() {
        if !result.contains(&row.category) {
            result.push(row.category.clone());
        }
    }

    result
}