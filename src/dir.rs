use crate::Progress;
use anyhow::Result;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
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

pub fn copy<S, D, P>(
    src: S,
    dest: D,
    exclude_paths: Vec<&str>,
    overwrite: bool,
    progress: Option<P>,
    buff_size: Option<usize>,
) -> Result<()>
where
    S: AsRef<Path>,
    D: AsRef<Path>,
    P: Fn(Progress),
{
    let paths = traverse(src.as_ref(), exclude_paths)?;

    let smth = src.as_ref();

    for path in paths {
        let src_path =
            PathBuf::from(src.as_ref()).join(path.to_str().unwrap().trim_start_matches("./"));
        let relative_path = path.strip_prefix(src.as_ref()).unwrap();
        let dest_path = PathBuf::from(dest.as_ref()).join(relative_path);

        println!("src: {src_path:?}\ndest: {dest_path:?}");

        if src_path.is_dir() {
            std::fs::create_dir_all(dest_path)?;
        } else {
            crate::file::copy(
                src_path,
                dest_path,
                overwrite,
                progress.as_ref().clone(),
                buff_size,
            )?;
        }
    }
    Ok(())
}
