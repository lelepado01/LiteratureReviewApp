use select::{document::Document, predicate::Name};


pub fn search_abstract(link_: String) -> String {
    let res = reqwest::get(link_);
    let res = pollster::block_on(res);
    if res.is_err() {
        return "Abstract not Found :(".to_string();
    }
    let body = res.unwrap().text();
    let body = pollster::block_on(body).unwrap(); 

    // select the abstract or the summary
    let abstract_ = Document::from(body.as_str())
        .find(Name("div")).filter(|n| {
            let text = n.inner_html().to_lowercase(); 
            text.contains("abstract") || text.contains("summary")
        })
        .map(|n| n.text())
        .collect::<Vec<_>>();

    if !abstract_.is_empty() {
        abstract_[0].clone()
    } else {
        "Abstract not Found :(".to_string()
    }
}

pub async fn search_abstract_async(link_: String) -> String {
    let res = reqwest::get(link_);
    let res = res.await;
    if res.is_err() {
        return "Abstract not Found :(".to_string();
    }
    let body = res.unwrap().text();
    let body = body.await.unwrap(); 

    // select the abstract or the summary
    let abstract_ = Document::from(body.as_str())
        .find(Name("div")).filter(|n| {
            let text = n.inner_html().to_lowercase(); 
            text.contains("abstract") || text.contains("summary")
        })
        .map(|n| n.text())
        .collect::<Vec<_>>();

    if !abstract_.is_empty() {
        abstract_[0].clone()
    } else {
        "Abstract not Found :(".to_string()
    }
}
