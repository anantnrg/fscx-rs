use std::path;

use walkdir::DirEntry;
use walkdir::WalkDir;

pub fn filter_paths(paths: Vec<&str>, item: &DirEntry) -> bool {
    for path in paths.clone() {
        if let Some(file_name) = item.path().to_str() {
            if file_name == path || file_name.contains(path) {
                return true;
            }
            // println!("{}", file_name);
        }
    }
    false
}

pub fn traverse(exclude_paths: Vec<&str>) -> Vec<path::PathBuf> {
    let mut paths = Vec::new();

    let walker = WalkDir::new("./").into_iter();
    for entry in walker.filter_entry(|e| !filter_paths(exclude_paths.clone(), e)) {
        paths.push(path::PathBuf::from(entry.unwrap().path()))
    }
    return paths;
}
