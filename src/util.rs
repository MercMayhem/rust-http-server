use std::error::Error;
use std::path::PathBuf;
use std::fs;

pub fn get_files_and_dirs(dir: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>>{
    let entities = fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .collect();

    return Ok(entities)
}

pub fn get_html_files(dir: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>>{
    let paths = fs::read_dir(dir)?
        .filter_map(|res| res.ok())
        .map(|dir_entry| dir_entry.path())
        .filter_map(|path| {
            if path.extension()
            .map_or(false, |ext| ext == "html"){
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(paths)
}

// Make this return Result instead
pub fn get_index_file(html_files: Vec<PathBuf>) -> Option<PathBuf>{
    let mut index_file = html_files
        .iter()
        .filter_map(
            |entry| if entry.file_stem().map_or(false, |name| name == "index"){
                Some(entry)
            } else {
                None
            }
        );

    return index_file.next().cloned();
}
