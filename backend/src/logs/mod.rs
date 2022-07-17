use crate::logs::storage::LogStorage;
use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Mutex;

mod storage;
mod types;

pub use types::LogEntry;

#[derive(Debug, Default)]
pub struct LogsState {
    storage: Mutex<LogStorage>,
}

/// Get all logs
///
/// Returns a list of all logs in storage.
#[utoipa::path(
responses(
(status = 200, description = "Successful retrieval", body = [LogEntry]),
)
)]
#[get("/api/logs")]
async fn get_logs(data: web::Data<LogsState>) -> impl Responder {
    let storage = data.storage.lock().unwrap();
    let logs = storage.get_all_logs();
    HttpResponse::Ok().json(logs)
}

pub(super) fn configure(state: web::Data<LogsState>) -> impl FnOnce(&mut web::ServiceConfig) {
    |config: &mut web::ServiceConfig| {
        config.app_data(state).service(get_logs);
    }
}
