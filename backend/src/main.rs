use actix_web::{get, http::header::ContentType, App, HttpResponse, HttpServer, Responder};

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
            .service(index)
            .configure(api_doc::config)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
