use crate::categories::categories_data::CategoryTag;
use crate::categories::categories_table::CategoriesTableRow;
use crate::export::export_pdf_table::ExportPDFTableRow;
use crate::dashboard::dashboard_table::DashboardTableRow;
use crate::general_memos::general_memo_data::Memo;
use crate::data::Paper;
use crate::paper_memos::paper_memo_data::PaperMemo;

use dioxus::prelude::GlobalAttributes;
use lopdf::{Document, Object};

static IGNORE: &[&str] = &[
    "Length",
    "BBox",
    "FormType",
    "Matrix",
    "Resources",
    "Type",
    "XObject",
    "Subtype",
    "Filter",
    "ColorSpace",
    "Width",
    "Height",
    "BitsPerComponent",
    "Length1",
    "Length2",
    "Length3",
    "PTEX.FileName",
    "PTEX.PageNumber",
    "PTEX.InfoDict",
    "FontDescriptor",
    "ExtGState",
    "Font",
    "MediaBox",
    "Annot",
];


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

pub fn load_dashboard_table_rows(name : &str) -> LoaderResult<Vec<DashboardTableRow>> {
    
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
                    || file_name.to_lowercase().contains(name) 
                    || author.to_lowercase().contains(name)
                    || categories.iter().any(|cat| cat.to_lowercase().contains(name))
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

fn filter_func(object_id: (u32, u16), object: &mut Object) -> Option<((u32, u16), Object)> {
    if IGNORE.contains(&object.type_name().unwrap_or_default()) {
        return None;
    }
    if let Ok(d) = object.as_dict_mut() {
        d.remove(b"Font");
        d.remove(b"Resources");
        d.remove(b"Producer");
        d.remove(b"ModDate");
        d.remove(b"Creator");
        d.remove(b"ProcSet");
        d.remove(b"XObject");
        d.remove(b"MediaBox");
        d.remove(b"Annots");
        if d.is_empty() {
            return None;
        }
    }
    Some((object_id, object.to_owned()))
}

pub fn load_pdf_content(file_name : &str) -> LoaderResult<String> {
    let doc = Document::load_filtered("papers/".to_owned() + file_name, filter_func); 

    match doc {
        Ok(doc) => {
            let mut content = String::new();
            for (page_num, _) in doc.get_pages() {
                let mut content_stream = String::new();
                if let Ok(content_object) = doc.extract_text(&[page_num]) {
                    content_stream.push_str(&content_object); 
                    content_stream.push('\n'); 
                }
                content.push_str(&content_stream);
            }
            LoaderResult::Ok(content)
        },
        Err(_) => {
            LoaderResult::Err(LoaderError::PdfExtract)
        }
    }

}

pub fn load_pdf_details(file_name : &str) -> LoaderResult<String> {
    let doc = Document::load_filtered("papers/".to_owned() + file_name, filter_func); 

    match doc {
        Ok(doc) => {
            for (_, page_id) in doc.get_pages() {
                let anns = doc.get_page_annotations(page_id);
                println!("{:?}", anns);
            }
            LoaderResult::Err(LoaderError::PdfExtract)
        },
        Err(_) => {
            LoaderResult::Err(LoaderError::PdfExtract)
        }
    }
}
 
pub fn load_paper_memos(query : &str) -> LoaderResult<Vec<PaperMemo>> {

    let file = std::fs::File::open("metadata/paper_memos.ron"); 
    if let Ok(file) = file {
        let memos: Result<Vec<PaperMemo>, ron::error::SpannedError> = ron::de::from_reader(file); 
        if let Ok(mut memos) = memos {
            memos.retain(|memo| memo.paper_name.to_lowercase().contains(query)); 
            LoaderResult::Ok(memos)
        } else {
            LoaderResult::Err(LoaderError::RonParse)
        }
    } else {
        LoaderResult::Err(LoaderError::FileOpen)
    }

}