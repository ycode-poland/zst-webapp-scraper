use actix_web::{
    HttpServer,
    App,
    web
};
use actix_cors::Cors;
use actix_web::middleware::Logger;

mod entities;
mod controllers;

mod utils;

use crate::controllers::plan_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(Logger::new("\"%a %s %r\" %b bytes %T s"))
            .wrap(cors)
            .route("/", web::get().to(controllers::home))
            .route("/plans", web::get().to(plan_controller::plans))
            .route("/plans/{id}", web::get().to(plan_controller::plan))
            // .service(
            //     web::scope("/api")
            //         .service(controllers::home)
            // )
    }).bind(("192.172.0.103", 3000))?
        .run()
        .await
}