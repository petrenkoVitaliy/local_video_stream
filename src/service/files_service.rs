use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use super::dtm::file_details::FileDetails;

pub struct FilesService {}

impl FilesService {
    fn get_files_list(path: &String) -> Result<Vec<String>, String> {
        let paths = match fs::read_dir(path) {
            Ok(files) => files,
            Err(error) => return Err(format!("cannot read video folder: {}", error)),
        };

        let mut files_list: Vec<String> = paths.into_iter().fold(vec![], |mut acc, path| {
            if let Ok(path) = path {
                if let Ok(metadata) = path.metadata() {
                    if !metadata.is_dir() {
                        acc.push(path.file_name().to_string_lossy().into_owned())
                    }
                }
            }

            acc
        });

        files_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        Ok(files_list)
    }

    pub fn get_video_files(videos_path: &String) -> Result<Vec<String>, String> {
        Self::get_files_list(&videos_path)
    }

    pub fn update_source_file(source_path: &String, videos_path: &String) -> Result<(), String> {
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .open(source_path)
        {
            Ok(file) => file,
            Err(error) => return Err(format!("cannot open videos path: {}", error)),
        };

        match file.write_all(videos_path.as_bytes()) {
            Ok(_) => Ok(()),
            Err(error) => return Err(format!("cannot write videos path: {}", error)),
        }
    }

    pub fn generate_cache(cache_path: &String) -> Result<HashMap<String, String>, String> {
        let cache_files = Self::get_files_list(cache_path)?;

        let cache = cache_files
            .into_iter()
            .fold(HashMap::new(), |mut acc, cache_file: String| {
                if let Ok(file_details) = Self::parse_video_name(&cache_file) {
                    acc.insert(file_details.name, cache_file);
                }

                acc
            });

        Ok(cache)
    }

    pub fn parse_video_name(filename: &String) -> Result<FileDetails, String> {
        let file = Path::new(filename);

        let extension = match file.extension() {
            Some(extension) => extension.to_string_lossy().into_owned(),
            None => return Err("cannot parse filename".to_string()),
        };
        let file_stem = match file.file_stem() {
            Some(file_stem) => file_stem.to_string_lossy().into_owned(),
            None => return Err("cannot parse filename".to_string()),
        };

        return Ok(FileDetails {
            ext: extension,
            name: file_stem,
        });
    }
}
