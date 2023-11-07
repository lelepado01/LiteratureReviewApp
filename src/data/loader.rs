use crate::categories;
use crate::categories::categories_data::CategoryTag;
use crate::categories::categories_table::CategoriesTableRow;
use crate::export::export_pdf_table::ExportPDFTableRow;
use crate::dashboard::dashboard_table::DashboardTableRow;
use crate::memos::memo_data::Memo;
use crate::data::Paper;


pub fn load_papers() -> Vec<Paper> {
    let file = std::fs::File::open("metadata/papers.ron").unwrap();
    let papers: Vec<Paper> = ron::de::from_reader(file).unwrap();

    papers
}

pub fn load_categories() -> Vec<CategoriesTableRow> {
    let file = std::fs::File::open("metadata/papers.ron").unwrap();
    let papers: Vec<Paper> = ron::de::from_reader(file).unwrap();
    
    let mut categories : Vec<CategoriesTableRow> = vec![]; 
    for paper in papers.iter() {
        for category in paper.categories.iter() {
            let mut found = false;
            for row in categories.iter_mut() {
                if row.category == *category {
                    row.paths.push(paper.file_name.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                categories.push(CategoriesTableRow {
                    category: category.clone(),
                    color: "red".to_string(),
                    paths: vec![paper.file_name.clone()],
                });
            }
        }
    }

    categories
}

pub fn load_memos() -> Vec<Memo> {
    let file = std::fs::File::open("metadata/memos.ron").unwrap();
    let memos: Vec<Memo> = ron::de::from_reader(file).unwrap();

    memos
}

pub fn load_categories_data() -> Vec<CategoryTag> {
    let file = std::fs::File::open("metadata/categories.ron").unwrap();
    let categories: Vec<CategoryTag> = ron::de::from_reader(file).unwrap();

    categories
}

pub fn load_dashboard_table_rows(name :String) -> Vec<DashboardTableRow> {
    
    let data = load_categories();

    let papers = std::fs::read_dir("./papers/").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>().unwrap();

    let mut result = Vec::new();

    for paper in papers.iter() {
        let file_name = paper.file_name().unwrap().to_str().unwrap().to_string();
        let author = "author".to_string(); // TODO: get author from pdf
        let pages = 1; // TODO: get pages from pdf
        let mut categories = Vec::new();

        for row in data.iter() {
            for path in row.paths.iter() {
                if path == &file_name && !categories.contains(&row.category) {
                    categories.push(row.category.clone());
                }
            }
        }

        if name.is_empty() 
            || file_name.to_lowercase().contains(&name) 
            || author.to_lowercase().contains(&name)
            || categories.iter().any(|cat| cat.to_lowercase().contains(&name))
            {
            result.push(DashboardTableRow {
                file_name,
                author,
                pages,
                categories,
            });
        }
    }

    result
}

pub fn load_pdf_export_rows() -> Vec<ExportPDFTableRow> {
    let file = std::fs::File::open("metadata/papers.ron").unwrap();
    let papers: Vec<Paper> = ron::de::from_reader(file).unwrap();

    let categories = load_categories_data();

    let mut result : Vec<ExportPDFTableRow> = vec![];
    for paper in papers.iter() {

        let mut paper_cats = vec![];
        for category in paper.categories.iter() {
            for cat in categories.iter() {
                if cat.label == *category {
                    paper_cats.push(cat.clone());
                }
            }
        }

        result.push(ExportPDFTableRow {
            file_name: paper.file_name.clone(),
            categories: paper_cats,
        });
    }

    result
}