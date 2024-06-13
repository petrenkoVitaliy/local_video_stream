use actix_web::web;
use std::fs::File;
use std::io::{Read, Seek};

use crate::module::sse_callback::PollCallback;
use crate::module::state::State;

use crate::dtm::chunk_details::ChunkDetails;
use crate::dtm::file_details::FileDetails;
use crate::dtm::file_format::VideoFormat;

use super::converter_service::ConverterService;

use super::files_service::FilesService;
use crate::utils::{get_file_path, get_file_path_with_ext, get_filename};

pub struct StreamService {}

impl StreamService {
    fn get_pipe_headers(chunk_details: &ChunkDetails, total: u64) -> Vec<(String, String)> {
        return vec![
            (
                String::from("Content-Range"),
                format!(
                    "bytes {}-{}/{}",
                    chunk_details.start, chunk_details.end, total
                ),
            ),
            (
                String::from("Content-Length"),
                format!("{}", chunk_details.size),
            ),
            (String::from("Content-Type"), String::from("video/mp4")),
            (String::from("Accept-Ranges"), String::from("bytes")),
        ];
    }

    fn get_chunk_details(range_header: &str, file_size: u64) -> ChunkDetails {
        const CHUNK_SIZE: u64 = 1_048_576; // 1 Mb

        let start_str: String = range_header.chars().filter(|c| c.is_digit(10)).collect();

        let start = start_str.parse::<u64>().ok().unwrap();
        let end = std::cmp::min(start + CHUNK_SIZE, file_size - 1);
        let size = end - start + 1;

        ChunkDetails { size, start, end }
    }

    fn get_stream_filename(
        state: &web::Data<State>,
        filename_details: &FileDetails,
    ) -> Result<Option<String>, String> {
        let stream_filename = match filename_details.ext.parse::<VideoFormat>() {
            Ok(VideoFormat::MP4) => Some(get_file_path_with_ext(
                &state.get_videos_path(),
                &filename_details.name,
                &filename_details.ext,
            )),
            _ => {
                let cached_filename = state.get_cached_filename(&filename_details.name);

                match cached_filename {
                    Some(cached_filename) => {
                        Some(get_file_path(&state.cache_path, &cached_filename))
                    }
                    _ => None,
                }
            }
        };

        Ok(stream_filename)
    }

    async fn convert_video(
        state: &web::Data<State>,
        filename_details: &FileDetails,
        query_filename: String,
        callback: Option<PollCallback>,
    ) -> Result<String, String> {
        let cache_filename = get_filename(&filename_details.name, VideoFormat::MP4.to_str());

        ConverterService::convert_video(
            &get_file_path(&state.cache_path, &cache_filename),
            &get_file_path_with_ext(
                &state.get_videos_path(),
                &filename_details.name,
                &filename_details.ext,
            ),
            query_filename,
            callback,
        )
        .await?;

        state.add_cache_item(&filename_details.name, &cache_filename);

        Ok(get_file_path(&state.cache_path, &cache_filename))
    }

    pub async fn get_video_stream(
        state: web::Data<State>,
        range_header: String,
        query_filename: String,
    ) -> Result<(Vec<u8>, Vec<(String, String)>), String> {
        let filename_details = FilesService::parse_video_name(&query_filename)?;

        let stream_filename = match Self::get_stream_filename(&state, &filename_details)? {
            Some(filename) => filename,
            None => Self::convert_video(&state, &filename_details, query_filename, None).await?,
        };

        let mut file = match File::open(stream_filename) {
            Ok(file) => file,
            Err(error) => return Err(format!("Cannot open file: {}", error)),
        };

        let file_size = match file.metadata() {
            Ok(file_metadata) => file_metadata.len(),
            Err(error) => return Err(format!("Cannot get file metadata: {}", error)),
        };

        let chunk_details = Self::get_chunk_details(&range_header, file_size);

        let headers = Self::get_pipe_headers(&chunk_details, file_size);

        let mut buffer = vec![0; chunk_details.size as usize];

        match file.seek(std::io::SeekFrom::Start(chunk_details.start)) {
            Err(error) => return Err(format!("Cannot read file in range: {}", error)),
            _ => match file.read_exact(&mut buffer) {
                Err(error) => return Err(format!("Cannot read file in range: {}", error)),
                _ => (),
            },
        }

        return Ok((buffer, headers));
    }

    pub async fn get_video_status(
        state: web::Data<State>,
        query_filename: String,
        callback: Option<PollCallback>,
    ) -> Result<String, String> {
        let filename_details = FilesService::parse_video_name(&query_filename)?;

        let stream_filename = match Self::get_stream_filename(&state, &filename_details)? {
            Some(filename) => {
                if let Some(callback) = callback {
                    let _ = callback(0);
                }

                filename
            }
            None => {
                Self::convert_video(&state, &filename_details, query_filename, callback).await?
            }
        };

        return Ok(stream_filename);
    }
}
