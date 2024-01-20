use walkdir::DirEntry;
use walkdir::WalkDir;

pub fn exclude_paths(paths: Vec<&str>, item: &DirEntry) -> bool {
    for path in &paths {
        if let Some(file_name) = item.file_name().to_str() {
            if file_name.starts_with(path) {
                return true;
            }
        }
    }
    false
}

pub fn traverse() {
    let paths = vec!["target", ".git"]; // Adjust with your exclude paths

    let walker = WalkDir::new("./").into_iter();
    for entry in walker.filter_entry(|e| !exclude_paths(paths.clone(), e)) {
        println!("{}", entry.unwrap().path().display());
    }
}
