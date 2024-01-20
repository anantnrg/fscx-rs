use std::path;

use walkdir::DirEntry;
use walkdir::WalkDir;

pub fn filter_paths(paths: Vec<&str>, item: &DirEntry) -> bool {
    for path in &paths {
        if let Some(file_name) = item.file_name().to_str() {
            if file_name.starts_with(path) {
                return true;
            }
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
