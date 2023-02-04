use actix_web::http::header::ContentType;
use actix_web::{
    HttpServer,
    App,
    web, HttpResponse
};
use actix_cors::Cors;
use actix_web::middleware::Logger;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

mod entities;
mod controllers;

mod utils;
use utils::str_convert::convert;

use crate::controllers::plan_controller::*;
use crate::entities::class::Class;

#[derive(Debug, Clone)]
pub struct AppState {
    pub class_list: Vec<Class>,
}

#[derive(Deserialize)]
struct Config {
    address: String,
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut file: File = File::open("config.json").unwrap();
    let mut data: String = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Config = serde_json::from_str(&data).unwrap();
	env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let bind: (&str, u16) = (convert(json.address.clone()), json.port);

    let state = AppState {
        class_list: vec![ Class { index: "".to_string(), name: "".to_string(), year: 0, } ],
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(Logger::new("\"%a %s %r\" %b bytes %T s"))
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .route("/", web::get().to(|| async { HttpResponse::Ok().content_type(ContentType::plaintext()).body("Thank you for using zst-webapp-scrapper by ycode.") }))
            .route("/plans", web::get().to(plans::plans))
            .route("/plans/{id}", web::get().to(plan::plan))
            .route("/announcements", web::get().to(controllers::announcements))
    }).bind(bind)?
        .run()
        .await
}
