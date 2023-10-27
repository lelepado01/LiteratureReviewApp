
pub fn update_categories() {
    let papers = std::fs::read_dir("./papers/").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>().unwrap();

    let filenames = papers.iter()
        .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
        .collect::<Vec<String>>();

    // write to papers.ron
    let mut file = std::fs::File::create("metadata/papers.ron").unwrap();
    ron::ser::to_writer(&mut file, &filenames).unwrap();
}