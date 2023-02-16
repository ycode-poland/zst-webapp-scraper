use std::collections::HashMap;

use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use scraper::Html;
use serde::Deserialize;
use serde_json::json;

use crate::{
    entities::table::{Column, PlanColumn, PlanRow, Subject},
    utils::{
        list::{get_id, IdType},
        scraper::{get_html, Scraper},
    },
};

#[derive(Deserialize, Debug, Clone)]
enum Type {
    Row,
    Column,
}

#[derive(Deserialize, Debug)]
pub struct Query {
    direction: Option<Type>,
}

impl Default for Type {
    fn default() -> Self {
        Self::Column
    }
}

impl Default for Query {
    fn default() -> Self {
        Self {
            direction: Some(Type::Column),
        }
    }
}

/**
 * @route GET /plan/:id
 */
pub async fn plan(path: web::Path<String>, query: web::Query<Query>) -> HttpResponse {
    let id = if path.as_ref()[0..1].parse::<u8>().is_ok() {
        get_id(&path.into_inner(), IdType::Class).await
    } else {
        get_id(&path.into_inner(), IdType::Teacher).await
    };

    let response = get_html(format!("http://www.zstrzeszow.pl/plan/plany/{id}.html"))
        .await
        .unwrap();

    let document = Html::parse_document(&response);
    let plans_s = ".tabela > tbody > tr".to_sel();
    let plans = document.select(&plans_s);

    let mut hours: Vec<String> = Vec::new();
    let mut lessons_column: HashMap<u8, Vec<Option<Column>>> = HashMap::new();
    let mut lessons_row: Vec<Vec<Option<Column>>> = Vec::new();

    let mut monday = Vec::new();
    let mut tuesday = Vec::new();
    let mut wednesday = Vec::new();
    let mut thursday = Vec::new();
    let mut friday = Vec::new();

    plans.zip(0..).for_each(|(item, i)| {
        if i > 0 {
            let hour = item.select(&"td.g".to_sel()).next().unwrap();
            let lesson_selector = "td.l".to_sel();
            let lesson_cells = item.select(&lesson_selector);

            let mut lessons: Vec<Option<Column>> = Vec::new();

            lesson_cells.zip(0..).for_each(|(cell, j)| {
                // If w zmiennej
                // Pozdrawiam tych co nie wierzyli :P
                let lesson: Option<Column> = if cell.inner_html() == "&nbsp;" {
                    None
                } else {
                    let group_check = cell
                        .select(&"br".to_sel())
                        .next()
                        .map_or_else(String::new, |n| n.html());

                    if group_check == "<br>" {
                        let mut subjects: Vec<Subject> = Vec::new();

                        let mut subjects_tmp = Vec::new();
                        let mut teachers = Vec::new();
                        let mut classrooms = Vec::new();

                        cell.select(&"span.p".to_sel())
                            .zip(0..)
                            .for_each(|(x, _num)| subjects_tmp.push(x.inner_html()));
                        cell.select(&".n".to_sel())
                            .zip(0..)
                            .for_each(|(x, _num)| teachers.push(x.inner_html()));
                        cell.select(&"span.s".to_sel())
                            .zip(0..)
                            .for_each(|(x, _num)| classrooms.push(x.inner_html()));

                        for (i, subject) in subjects_tmp.iter().enumerate() {
                            subjects.push(Subject {
                                subject: subject.clone(),
                                teacher: teachers.get(i).unwrap_or(&String::new()).clone(),
                                classroom: classrooms.get(i).unwrap_or(&String::new()).clone(),
                            });
                        }
                        Some(Column {
                            lesson_number: j,
                            subjects,
                        })
                    } else {
                        let subject = cell.select(&"span.p".to_sel()).next().map_or_else(
                            || cell.inner_html().split(' ').collect::<Vec<&str>>()[1].to_owned(),
                            |v| v.inner_html(),
                        );
                        let teacher = cell.select(&".n".to_sel()).next().map_or_else(
                            || cell.inner_html().split(' ').collect::<Vec<&str>>()[0].to_owned(),
                            |v| v.inner_html(),
                        );
                        let classroom = cell.select(&"span.s".to_sel()).next().map_or_else(
                            || {
                                let cell = cell.inner_html();
                                let vec = cell.split(' ').collect::<Vec<&str>>();
                                let len = vec.len();
                                vec[len - 1].to_owned()
                            },
                            |v| v.inner_html(),
                        );

                        Some(Column {
                            lesson_number: i,
                            subjects: vec![Subject {
                                subject,
                                teacher,
                                classroom,
                            }],
                        })
                    }
                };

                match query.direction.clone().unwrap_or_default() {
                    Type::Column => match j {
                        0 => monday.push(lesson),
                        1 => tuesday.push(lesson),
                        2 => wednesday.push(lesson),
                        3 => thursday.push(lesson),
                        4 => friday.push(lesson),
                        _ => {}
                    },
                    Type::Row => {
                        lessons.push(lesson);
                    }
                };
            });

            if let Type::Row = query.direction.clone().unwrap_or_default() {
                lessons_row.push(lessons);
            }

            hours.push(hour.inner_html().replace(' ', ""));
        }
    });

    return match query.direction.clone().unwrap_or_default() {
        Type::Column => {
            lessons_column.insert(0, monday);
            lessons_column.insert(1, tuesday);
            lessons_column.insert(2, wednesday);
            lessons_column.insert(3, thursday);
            lessons_column.insert(4, friday);

            let plan = PlanColumn {
                hours,
                weekdays: lessons_column,
            };

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(json! { plan })
        }
        Type::Row => {
            let plan = PlanRow {
                hours,
                weekdays: lessons_row,
            };

            HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(json! { plan })
        }
    };
}
