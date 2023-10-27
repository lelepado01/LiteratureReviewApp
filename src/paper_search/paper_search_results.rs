use select::{document::Document, predicate::{Predicate, Class, Name}};


#[derive(Clone, Debug, PartialEq)]
pub struct PaperSearchResult {
    pub file_name: String,
    pub link: String,
    pub file_content: String,
    pub author: String,
    pub year: String,
}



pub fn search_paper_online(query: &str) -> Vec<PaperSearchResult> {

    let res = reqwest::get("https://scholar.google.com/scholar?hl=en&as_sdt=0%2C5&q=".to_owned() + query.replace(" ", "+").as_str()); 
    let res = pollster::block_on(res).unwrap();
    let body = res.text();
    let body = pollster::block_on(body).unwrap(); 

    let titles = Document::from(body.as_str())
        .find(Name("h3").and(Class("gs_rt")).child(Name("a")))
        .map(|n| n.text())
        .collect::<Vec<_>>()[..10].to_vec();

    let links = Document::from(body.as_str())
        .find(Name("h3").and(Class("gs_rt")).child(Name("a")))
        .map(|n| n.attr("href").unwrap_or_default().to_string())
        .collect::<Vec<_>>();

    let mut authors = Document::from(body.as_str())
        .find(Name("div").and(Class("gs_a")))
        .map(|n| n.text())
        .collect::<Vec<_>>();

    let year_re = regex::Regex::new(r"\d{4}").unwrap();
    let mut years = vec![]; 
    for author in authors.iter(){
        let mut matches = year_re.find_iter(author); 
        if let Some(mat) = matches.next() {
            years.push(mat.as_str().to_string()); 
        } else {
            years.push("".to_string());
        }
    }

    for author in authors.iter_mut() {
        *author = author.split('-').next().unwrap_or_default().to_string();
        *author = author.split(" - ").next().unwrap_or_default().to_string();
        *author = author.split('…').next().unwrap_or_default().to_string();
    }

    let intro = Document::from(body.as_str())
        .find(Name("div").and(Class("gs_rs")))
        .map(|n| n.text())
        .collect::<Vec<_>>();

    let mut results = vec![];
    for i in 0..intro.len() {
        results.push(PaperSearchResult {
            file_name: titles[i].clone(),
            link: links[i].clone(),
            file_content: intro[i].clone(),
            author: authors[i].clone(),
            year: years[i].clone(),
        }); 
    }

    results
}