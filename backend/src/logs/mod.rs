use crate::logs::storage::LogStorage;
use actix_web::{get, post, web, HttpResponse, Responder};
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

/// Add a log
///
/// Add a log to storage
#[utoipa::path(
request_body = LogEntry,
responses(
(status = 200, description = "Successful addition. Create log entry is returned (including new id)", body = LogEntry),
)
)]
#[post("/api/logs")]
async fn add_log(new_log: web::Json<LogEntry>, data: web::Data<LogsState>) -> impl Responder {
    let mut storage = data.storage.lock().unwrap();

    let response = storage.add_log(new_log.into_inner());
    HttpResponse::Ok().json(response)
}

pub(super) fn configure(state: web::Data<LogsState>) -> impl FnOnce(&mut web::ServiceConfig) {
    |config: &mut web::ServiceConfig| {
        config.app_data(state).service(get_logs).service(add_log);
    }
}
