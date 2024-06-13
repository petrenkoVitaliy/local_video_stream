use regex::Regex;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

use crate::dtm::file_format::VideoFormat;
use crate::module::sse_callback::PollCallback;
use crate::utils::get_ms_from_capture;

pub struct ConverterService {}

impl ConverterService {
    fn get_conversion_progress(duration: u64, converted_time_ms: u64) -> u64 {
        if duration == 0 {
            return 0;
        }

        ((converted_time_ms as f64 / duration as f64) * 100.0).round() as u64
    }

    async fn read_output<R>(
        mut reader: R,
        query_filename: String,
        callback: Option<PollCallback>,
    ) -> Result<(), String>
    where
        R: tokio::io::AsyncRead + std::marker::Unpin,
    {
        let mut buffer = [0u8; 1024];

        let duration_regex = Regex::new(r"Duration: (\d+):(\d+):(\d+)\.(\d+)").unwrap();
        let progress_regex = Regex::new(r"time=(\d+):(\d+):(\d+)\.(\d+)").unwrap();

        let mut duration: u64 = 0;

        loop {
            let bytes_read = reader.read(&mut buffer).await.unwrap();

            if bytes_read == 0 {
                break;
            }

            let output_chunk = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            if duration == 0 {
                if let Some(captures) = duration_regex.captures(&output_chunk) {
                    duration = get_ms_from_capture(captures);
                }
            }

            if let Some(captures) = progress_regex.captures(&output_chunk) {
                let converted_time_ms = get_ms_from_capture(captures);
                let progress = Self::get_conversion_progress(duration, converted_time_ms);

                println!("Progress: {} : {}%", query_filename, progress);

                if let Some(callback) = &callback {
                    let _ = callback(progress).await;
                }
            }
        }

        println!("Completed: {} ", query_filename);

        Ok(())
    }

    pub async fn convert_video(
        cache_file: &str,
        source_file: &str,
        query_filename: String,
        callback: Option<PollCallback>,
    ) -> Result<(), String> {
        let mut ffmpeg = Command::new("ffmpeg")
            .arg("-i")
            .arg(source_file)
            .arg("-movflags")
            .arg("+faststart")
            .arg("-c:v")
            .arg("copy")
            .arg("-c:a")
            .arg("aac")
            .arg("-f")
            .arg(VideoFormat::MP4.to_str())
            .arg(cache_file)
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("cannot start ffmpeg converting");

        let stderr = ffmpeg.stderr.take().expect("Failed to open stderr");
        let read_stderr = Self::read_output(stderr, query_filename, callback);

        tokio::spawn(read_stderr);

        match ffmpeg.wait().await {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }
}
