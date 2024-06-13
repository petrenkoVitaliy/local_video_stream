use actix_web::{get, put, web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use serde::Deserialize;

use crate::dtm::source_payload::SourcePayload;

use crate::module::event_stream::EventStream;
use crate::module::sse_callback::get_callback;
use crate::module::state::State;

use super::template::IndexTemplate;

use super::service::files_service::FilesService;
use super::service::stream_service::StreamService;

#[derive(Deserialize)]
struct Info {
    q: String,
}

#[get("/")]
pub async fn get_index_page(state: web::Data<State>) -> impl Responder {
    let videos_path = state.get_videos_path();

    let video_files = match FilesService::get_video_files(&videos_path) {
        Ok(video_files) => video_files,
        Err(error) => {
            return HttpResponse::InternalServerError().body(error);
        }
    };

    let cache = match FilesService::generate_cache(&state.cache_path) {
        Ok(result) => result,
        Err(error) => {
            return HttpResponse::InternalServerError().body(error);
        }
    };

    state.update_cache(cache);

    let template = IndexTemplate {
        video_files: &video_files,
        video_source: &videos_path,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[get("/chunk")]
pub async fn pipe_video_stream(
    request: HttpRequest,
    state: web::Data<State>,
    info: web::Query<Info>,
) -> impl Responder {
    let range_header = match request.headers().get("range") {
        Some(range_header) => match range_header.to_str() {
            Ok(range_header) => range_header.to_string(),
            Err(error) => {
                return HttpResponse::BadRequest().body(error.to_string());
            }
        },
        None => {
            return HttpResponse::BadRequest().body("range header is missing");
        }
    };

    let query = info.q.to_owned();

    let (buffer, headers) =
        match StreamService::get_video_stream(state, range_header.to_string(), query).await {
            Ok(result) => result,
            Err(error) => {
                return HttpResponse::InternalServerError().body(error);
            }
        };

    let mut response = HttpResponse::PartialContent();

    for (key, value) in headers {
        response.insert_header((key, value));
    }

    response.body(buffer)
}

#[get("/status")]
pub async fn pipe_video_status(state: web::Data<State>, info: web::Query<Info>) -> impl Responder {
    let query = info.q.to_owned();

    let (rx, callback) = get_callback();

    tokio::spawn(
        async move { StreamService::get_video_status(state, query, Some(callback)).await },
    );

    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Cache-Control", "no-cache"))
        .streaming(EventStream { rx })
}

#[put("/source")]
pub async fn update_source(
    state: web::Data<State>,
    payload: web::Json<SourcePayload>,
) -> impl Responder {
    if state.get_videos_path() == payload.source {
        return HttpResponse::InternalServerError().finish();
    }

    match FilesService::get_video_files(&payload.source) {
        Ok(_) => match FilesService::update_source_file(&state.source_path, &payload.source) {
            Ok(_) => state.update_videos_path(payload.source.clone()),
            Err(error) => {
                return HttpResponse::InternalServerError().body(error);
            }
        },
        Err(error) => {
            return HttpResponse::InternalServerError().body(error);
        }
    };

    HttpResponse::Ok().finish()
}
