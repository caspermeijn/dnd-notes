use serde::Deserialize;
use serde::Serialize;
use utoipa::Component;

/// A logged fact.
#[derive(Deserialize, Serialize, Component, Clone, Debug)]
pub struct LogEntry {
    /// Unique identifier for the log entry.
    #[component(example = 1234)]
    pub id: u32,
    /// Textual description of the fact that is logged.
    #[component(example = "Lord Strahd lives in castle Ravenloft")]
    pub text: String,
    /// List of tags accosiated to the logged fact.
    #[component(example = json!(["Strahd", "Ravenloft"]))]
    pub tags: Vec<String>,
}
