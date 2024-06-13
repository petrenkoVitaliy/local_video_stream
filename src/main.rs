mod dtm;
mod module;
mod routes;
mod service;
mod template;
mod utils;

use actix_web::{web, App, HttpServer};
use local_ip_address::local_ip;
use std::{collections::HashMap, sync::Mutex};
use utils::read_lines;

use crate::module::state::State;
use routes::{get_index_page, pipe_video_status, pipe_video_stream, update_source};

fn read_videos_path(path: &str) -> Option<String> {
    let lines = match read_lines(path) {
        Ok(lines) => lines,
        Err(_) => return None,
    };

    lines.flatten().last()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 8080;

    const SOURCE_PATH: &str = "/home/salander/.local_video/path.txt";
    const CACHE_PATH: &str = "/home/salander/.local_video/cache";
    const DEFAULT_PATH: &str = "/home";

    let videos_path = read_videos_path(SOURCE_PATH).unwrap_or(DEFAULT_PATH.to_string());

    let cache_path = utils::get_folder_path(CACHE_PATH);

    let state: web::Data<State> = web::Data::new(State {
        cache_path,
        source_path: SOURCE_PATH.to_string(),
        cache: Mutex::new(HashMap::new()),
        videos_path: Mutex::new(videos_path),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(pipe_video_stream)
            .service(get_index_page)
            .service(pipe_video_status)
            .service(update_source)
    })
    .bind(("0.0.0.0", PORT))?
    .run();

    println!("Server started at: {}:{}", local_ip().unwrap(), PORT);

    server.await
}
