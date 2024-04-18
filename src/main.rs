use lib::controllers;
use lib::controllers::plan_controller::{classrooms, news, plan, plans, teachers};
use lib::entities::class::Class;
use lib::entities::table::PlanRow;
use lib::utils::str_convert::convert;
use lib::{ApiError, AppState, Config};

use actix_cors::Cors;
use actix_web::http::header::ContentType;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use sqlx::sqlite::SqlitePoolOptions;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> Result<(), ApiError> {
    let mut file: File = File::open("config.json")?;
    let mut data: String = String::new();
    file.read_to_string(&mut data)?;
    let json: Config = serde_json::from_str(&data)?;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let bind: (&str, u16) = (convert(json.address.clone()), json.port);

    let state = AppState {
        class_list: vec![Class {
            index: String::new(),
            name: String::new(),
            year: 0,
        }],
        plan: Arc::new(Mutex::new(HashMap::new())),
    };

    let pool = SqlitePoolOptions::new()
        .connect("sqlite://sqlite.db")
        .await
        .unwrap();

    let res = sqlx::query!("SELECT * FROM classes")
        .fetch_all(&pool)
        .await
        .unwrap();

    for row in res.iter() {
        state.plan.lock().unwrap().insert(
            row.class_number.clone().unwrap(),
            serde_json::from_str::<PlanRow>(&row.table_data.clone().unwrap()).unwrap(),
        );
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(Logger::new("\"%a %s %r\" %b bytes %T s"))
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .route(
                "/",
                web::get().to(|| async {
                    HttpResponse::Ok()
                        .content_type(ContentType::plaintext())
                        .body("Thank you for using zst-webapp-scrapper by ycode.")
                }),
            )
            .route("/plans", web::get().to(plans::plans))
            .route("/plans/{id}", web::get().to(plan::plan))
            .route("/teachers", web::get().to(teachers::teachers))
            .route("/classrooms/{id}", web::get().to(classrooms::classrooms))
            .route("/announcements", web::get().to(controllers::announcements))
            .route("/news", web::get().to(news::news))
    })
    .bind(bind)?
    .run()
    .await?;
    Ok(())
}
