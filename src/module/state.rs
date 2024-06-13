use std::{collections::HashMap, sync::Mutex};

#[derive(Debug)]
pub struct State {
    pub videos_path: Mutex<String>,
    pub cache_path: String,
    pub source_path: String,
    pub cache: Mutex<HashMap<String, String>>,
}

impl State {
    pub fn update_cache(&self, new_cache: HashMap<String, String>) -> () {
        let mut cache = self.cache.lock().expect("Cannot lock cache");

        *cache = new_cache;
    }

    pub fn update_videos_path(&self, new_videos_path: String) -> () {
        let mut videos_path = self.videos_path.lock().expect("Cannot lock videos path");

        *videos_path = new_videos_path;
    }

    pub fn get_videos_path(&self) -> String {
        let videos_path = self.videos_path.lock().expect("Cannot lock videos path");

        videos_path.clone()
    }

    pub fn add_cache_item(
        &self,
        origin_filename: &String,
        cache_filename: &String,
    ) -> Option<String> {
        let mut cache = self.cache.lock().expect("Cannot lock cache");

        cache.insert(origin_filename.to_string(), cache_filename.to_string())
    }

    pub fn get_cached_filename(&self, origin_filename: &str) -> Option<String> {
        let cache = self.cache.lock().expect("Cannot lock cache");

        cache.get(origin_filename).cloned()
    }
}
