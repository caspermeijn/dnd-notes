use serde::Deserialize;
use serde::Serialize;
use utoipa::Component;

/// A logged fact.
#[derive(Deserialize, Serialize, Component, Clone, Debug)]
pub struct LogEntry {
    /// Unique identifier for the log entry.
    #[component(read_only, example = 1234)]
    #[serde(default)]
    pub id: u32,
    /// Textual description of the fact that is logged.
    #[component(example = "Lord Strahd lives in castle Ravenloft")]
    pub text: String,
    /// List of tags accosiated to the logged fact.
    #[component(example = "Strahd")]
    //TODO: Add "Ravenloft" as second tag https://github.com/juhaku/utoipa/issues/225
    pub tags: Vec<String>,
}
