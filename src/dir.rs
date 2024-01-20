use anyhow::Result;
use std::path;
use std::path::Path;
use walkdir::DirEntry;
use walkdir::WalkDir;

pub fn filter_paths(paths: Vec<&str>, item: &DirEntry) -> bool {
    for &path in &paths {
        if let Some(file_name) = item.path().to_str() {
            if file_name == path || file_name.contains(path) {
                return true;
            }
        }
    }
    false
}

pub fn traverse<D>(dir: D, exclude_paths: Vec<&str>) -> Result<Vec<path::PathBuf>>
where
    D: AsRef<Path>,
{
    let mut paths = Vec::new();

    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_entry(|e| !filter_paths(exclude_paths.clone(), e)) {
        paths.push(path::PathBuf::from(entry.unwrap().path()))
    }
    Ok(paths)
}
