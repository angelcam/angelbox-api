// Copyright 2021 Angelcam, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod api;
mod config;
mod logger;
mod speaker;

use std::io;

use actix_web::{http::StatusCode, middleware, web, App, HttpResponse, HttpServer};

use crate::config::Config;

static HOME_HTML: &[u8] = include_bytes!("html/home.html");

/// Root endpoint.
async fn home() -> HttpResponse {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html")
        .body(HOME_HTML)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let config = Config::new();

    logger::init(&config);

    let app_factory = || {
        App::new()
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(home)))
            .service(web::scope("/api/v1").configure(api::configure))
    };

    HttpServer::new(app_factory)
        .bind(config.bind_address())?
        .run()
        .await
}
