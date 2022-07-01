use actix_web::{
    get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::Component;

mod api_doc;

/// Get a hello message
#[utoipa::path(
    responses(
        (status = 200, description = "Textual hello", body = String),
    )
)]
#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize, Serialize, Component, Clone, Debug)]
pub struct AdditionalNames {
    others: Vec<String>,
}

/// Get a customized hello
#[utoipa::path(
    request_body = Option<AdditionalNames>,
    responses(
        (status = 200, description = "Textual hello", body = String),
    )
)]
#[post("/api/hello/{name}")]
async fn hello_name(
    name: web::Path<String>,
    additional: Option<web::Json<AdditionalNames>>,
) -> impl Responder {
    let other_names = if let Some(additional) = additional {
        additional.others.join(", ")
    } else {
        String::new()
    };

    if other_names.is_empty() {
        format!("Hello {name}!")
    } else {
        format!("Hello {name} and welcome to {other_names}!")
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body("Welcome to dnd-notes backend server. Try <a href=\"/api/hello\">hello</a> api. Or discover the api via <a href=\"/api-doc/swagger-ui/\">swagger</a>.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = dotenv::var("PORT")
        .expect("Environment variable PORT is required")
        .parse()
        .expect("Environment variable PORT must be a number");

    println!(
        "Starting dnd-notes backend service on http://127.0.0.1:{}",
        port
    );

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_name)
            .service(index)
            .configure(api_doc::config)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
