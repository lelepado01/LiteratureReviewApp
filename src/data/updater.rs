use crate::data::loader::load_papers;


pub fn update_categories(file_name: &str, label: &str) {
    let mut papers = load_papers();

    for paper in papers.iter_mut() {
        if paper.file_name == file_name {
            let mut found = false;
            for category in paper.categories.iter() {
                if category == &label {
                    found = true;
                }
            }
            if !found {
                paper.categories.push(label.to_string());
            } else {
                paper.categories.retain(|cat| cat != &label);
            }
        }
    }

    let mut file = std::fs::File::create("metadata/papers.ron").unwrap();
    ron::ser::to_writer(&mut file, &papers).unwrap();
}