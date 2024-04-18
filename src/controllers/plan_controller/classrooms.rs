use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::sqlite::SqlitePoolOptions;

use crate::{
    entities::{
        class::Class,
        table::{Column, PlanRow},
    },
    ApiError, AppState,
};

/**
 * @route GET /classrooms
 */
pub async fn classrooms(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let room = path.into_inner();
    if let Some(plan) = state.plan.lock().unwrap().get(&room) {
        return Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(json! { plan }));
    }

    let classes = reqwest::get("http://localhost:3000/plans")
        .await?
        .json::<Vec<Class>>()
        .await?;

    let mut final_plan: PlanRow = PlanRow {
        hours: vec![],
        weekdays: vec![],
    };

    for _ in 0..13 {
        let lessons: Vec<Option<Column>> = vec![None; 12];
        final_plan.weekdays.push(lessons);
    }

    for v in classes {
        let plan = reqwest::get(format!(
            "http://localhost:3000/plans/{}{}?direction=Row",
            v.year, v.name
        ))
        .await?
        .json::<PlanRow>()
        .await?;

        for (i, weekday) in plan.weekdays.iter().enumerate() {
            // println!("{:?}", weekday);
            for (j, lesson) in weekday.iter().enumerate() {
                if let Some(lesson) = lesson {
                    let classroom: Vec<String> = lesson
                        .subjects
                        .iter()
                        .map(|v| v.classroom.clone().unwrap_or("".to_string()))
                        .collect();
                    if classroom.contains(&room.to_string()) {
                        final_plan.weekdays[i][j] = Some(lesson.to_owned());
                    }
                }
            }
        }
    }

    let pool = SqlitePoolOptions::new()
        .connect("sqlite://sqlite.db")
        .await
        .unwrap();

    sqlx::query(
        format!(
            "INSERT INTO classes(class_number, table_data) VALUES ('{}', '{}')",
            room,
            serde_json::to_string(&final_plan).unwrap()
        )
        .as_str(),
    )
    .execute(&pool)
    .await
    .unwrap();

    state.plan.lock().unwrap().insert(room, final_plan.clone());

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(json! { final_plan }))
}
