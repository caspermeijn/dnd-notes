/* Copyright (C) 2022 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::logs::LogsState;
use actix_cors::Cors;
use actix_web::{
    get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use utoipa::Component;

mod api_doc;
mod logs;

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

    let log_state = web::Data::new(LogsState::default());

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(cors)
            .service(hello)
            .service(hello_name)
            .service(index)
            .configure(api_doc::config)
            .configure(logs::configure(log_state.clone()))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
