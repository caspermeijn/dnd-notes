use crate::logs::types::LogEntry;

#[derive(Clone, Debug)]
pub struct LogStorage {
    logs: Vec<LogEntry>,
}

impl LogStorage {
    pub fn get_all_logs(&self) -> Vec<LogEntry> {
        self.logs.clone()
    }
}

impl Default for LogStorage {
    fn default() -> Self {
        LogStorage {
            logs: vec![
                LogEntry {
                    id: 1,
                    text: "Strahd woont in Ravenloft".to_string(),
                    tags: vec!["Strahd".to_string(), "Ravenloft".to_string()],
                },
                LogEntry {
                    id: 2,
                    text: "Kasteel Ravenloft staat in de regio Barovia".to_string(),
                    tags: vec!["Barovia".to_string(), "Ravenloft".to_string()],
                },
                LogEntry {
                    id: 3,
                    text: "Strahd regeert over de regio Barovia".to_string(),
                    tags: vec!["Strahd".to_string(), "Barovia".to_string()],
                },
                LogEntry {
                    id: 4,
                    text: "Strahd heeft 4 bruiden".to_string(),
                    tags: vec!["Strahd".to_string()],
                },
            ],
        }
    }
}
