use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use regex::Captures;

pub fn get_folder_path(folder: &str) -> String {
    let videos_dir = &PathBuf::from(folder);

    fs::canonicalize(videos_dir)
        .expect("cannot parse path")
        .to_string_lossy()
        .into_owned()
}

pub fn get_file_path_with_ext(folder: &str, filename: &str, ext: &str) -> String {
    format!("{}/{}.{}", folder, filename, ext)
}

pub fn get_file_path(folder: &str, filename: &str) -> String {
    format!("{}/{}", folder, filename)
}

pub fn get_filename(filename: &str, ext: &str) -> String {
    format!("{}.{}", filename, ext)
}

pub fn get_ms_from_capture(captures: Captures) -> u64 {
    let hours: u64 = captures[1].parse().unwrap();
    let minutes: u64 = captures[2].parse().unwrap();
    let seconds: u64 = captures[3].parse().unwrap();
    let milliseconds: u64 = captures[4].parse().unwrap();

    let duration_ms = hours * 3600000 + minutes * 60000 + seconds * 1000 + milliseconds;

    duration_ms
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
