use crate::logs::types::LogEntry;
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct LogStorage {
    logs: Vec<LogEntry>,
    next_id: u32,
}

impl LogStorage {
    pub fn get_all_logs(&self) -> Vec<LogEntry> {
        self.logs.clone()
    }

    pub fn add_log(&mut self, partial_log: LogEntry) -> LogEntry {
        let mut complete_log = partial_log;
        complete_log.id = self.next_id;
        self.next_id += 1;

        self.logs.push(complete_log.clone());

        complete_log
    }

    pub fn get_all_tags(&self) -> Vec<String> {
        self.logs
            .iter()
            .flat_map(|log| log.tags.clone())
            .unique()
            .collect()
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
            next_id: 5,
        }
    }
}
