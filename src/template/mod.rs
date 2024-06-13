use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub video_files: &'a Vec<String>,
    pub video_source: &'a String,
}
