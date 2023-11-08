use crate::categories::categories_data::CategoryTag;
use crate::categories::categories_table::CategoriesTableRow;
use crate::export::export_pdf_table::ExportPDFTableRow;
use crate::dashboard::dashboard_table::DashboardTableRow;
use crate::memos::memo_data::Memo;
use crate::data::Paper;

pub enum LoaderResult<T> {
    Ok(T), 
    Err(LoaderError), 
}

#[derive(Debug)]
pub enum LoaderError {
    FileOpen,
    RonParse,
    DataElaboration,
    PdfExtract,
}

pub fn load_papers() -> LoaderResult<Vec<Paper>> {
    let file = std::fs::File::open("metadata/papers.ron"); 
    if let Ok(file) = file {
        let papers = ron::de::from_reader(file); 
        if let Ok(papers) = papers {
            LoaderResult::Ok(papers)
        } else {
            LoaderResult::Err(LoaderError::RonParse)
        }
    } else {
        LoaderResult::Err(LoaderError::FileOpen)
    }
}

pub fn load_paper_files() -> LoaderResult<Vec<String>> {
    let pps = std::fs::read_dir("papers"); 

    match pps {
        Ok(pps) => {
            let mut papers = Vec::new();
            for pp in pps {
                let pp = pp.unwrap();
                let file_name = pp.file_name().into_string().unwrap();
                papers.push(file_name);
            }
            LoaderResult::Ok(papers)
        },
        Err(_) => {
            LoaderResult::Err(LoaderError::FileOpen)
        }
    }
}

pub fn load_categories() -> LoaderResult<Vec<CategoriesTableRow>> {
    let papers = load_papers();
    match papers {
        LoaderResult::Ok(papers) => {
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

            LoaderResult::Ok(categories)
        },
        _ => {
            LoaderResult::Err(LoaderError::DataElaboration)
        }
    }
}

pub fn load_memos() -> LoaderResult<Vec<Memo>> {
    let file = std::fs::File::open("metadata/memos.ron"); 
    if let Ok(file) = file {
        let memos = ron::de::from_reader(file); 
        if let Ok(memos) = memos {
            LoaderResult::Ok(memos)
        } else {
            LoaderResult::Err(LoaderError::RonParse)
        }
    } else {
        LoaderResult::Err(LoaderError::FileOpen)
    }
}

pub fn load_categories_data() -> LoaderResult<Vec<CategoryTag>> {
    let file = std::fs::File::open("metadata/categories.ron"); 
    if let Ok(file) = file {
        let categories = ron::de::from_reader(file); 
        if let Ok(categories) = categories {
            LoaderResult::Ok(categories)
        } else {
            LoaderResult::Err(LoaderError::RonParse)
        }
    } else {
        LoaderResult::Err(LoaderError::FileOpen)
    }
}

pub fn load_dashboard_table_rows(name :String) -> LoaderResult<Vec<DashboardTableRow>> {
    
    let papers = std::fs::read_dir("./papers/").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>().unwrap();

    let data = load_categories();
    match data {
        LoaderResult::Ok(data) => {
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

            LoaderResult::Ok(result)
        },
        _ => {
            LoaderResult::Err(LoaderError::DataElaboration)
        }
    }
}

pub fn load_pdf_export_rows() -> LoaderResult<Vec<ExportPDFTableRow>> {
    let papers = load_papers(); 
    let categories = load_categories_data();

    match (papers, categories) {
        (LoaderResult::Ok(papers), LoaderResult::Ok(categories)) => {
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

            LoaderResult::Ok(result)
        },
        _ => {
            LoaderResult::Err(LoaderError::DataElaboration)
        }
    }
}

pub fn load_pdf_content(file_name : &str) -> LoaderResult<String> {
    let bytes = std::fs::read("papers/".to_owned() + file_name); 
    if let Ok(bytes) = bytes  {
        let extracted = pdf_extract::extract_text_from_mem(&bytes); 
        if let Ok(extracted) = extracted {
            LoaderResult::Ok(extracted)
        } else {
            LoaderResult::Err(LoaderError::PdfExtract)
        }
    } else {
        LoaderResult::Err(LoaderError::FileOpen)
    }
}