use crate::categories::categories_table::CategoriesTableRow;
use crate::export::export_pdf_table::ExportPDFTableRow;

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

pub fn load_pdf_export_rows() -> Vec<ExportPDFTableRow> {
    let file = std::fs::File::open("metadata/categories.ron").unwrap();
    let categories: Vec<CategoriesTableRow> = ron::de::from_reader(file).unwrap();

    let mut result = Vec::new();

    for row in categories.iter() {
        let mut paths = Vec::new();
        for path in row.paths.iter() {
            paths.push(path.clone());
        }
        result.push(ExportPDFTableRow {
            category: row.category.clone(),
            paths,
        });
    }

    result
}