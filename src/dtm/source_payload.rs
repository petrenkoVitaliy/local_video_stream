use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SourcePayload {
    pub source: String,
}
