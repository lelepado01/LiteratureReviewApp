
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

pub fn get_paper_citation(title : String) -> String {
    let res = reqwest::get("http://httpbin.org/get");

    let page = pollster::block_on(res).unwrap(); 

    println!("Status: {}", page.status());
    println!("Headers:\n{:#?}", page.headers());

    let body = pollster::block_on(page.text()).unwrap();
    println!("Body:\n{}", body);

    let cits = Document::from(body.as_str())
        .find(Name("div").and(Class("gs_fl")))
        .filter(|n| n.text().contains("Cited by"))
        .map(|n| n.text())
        .collect::<Vec<_>>(); 

    println!("{:?}", cits);

    body
}