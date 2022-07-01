use actix_web::{get, App, HttpServer, Responder, HttpResponse, http::header::ContentType};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
    .content_type(ContentType::html())
    .body("Welcome to dnd-notes backend server. Try <a href=\"/hello\">hello</a> api.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = dotenv::var("PORT")
        .expect("Environment variable PORT is required")
        .parse()
        .expect("Environment variable PORT must be a number");

    println!("Starting dnd-notes backend service on http://127.0.0.1:{}", port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(index)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}