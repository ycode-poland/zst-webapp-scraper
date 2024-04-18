use std::collections::HashMap;

use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Responder};
use scraper::{ElementRef, Html};
use serde::Deserialize;
use serde_json::json;

use crate::{
    entities::table::{Column, PlanColumn, PlanRow, Subject},
    utils::{
        list::{get_id, IdType},
        scraper::{get_html, Scraper},
    },
    ApiError,
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

fn with_group(cell: ElementRef<'_>, j: u8) -> Option<Column> {
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
            teacher: Some(teachers.get(i).unwrap_or(&String::new()).clone()),
            classroom: Some(classrooms.get(i).unwrap_or(&String::new()).clone()),
        });
    }
    Some(Column {
        lesson_number: j,
        subjects,
    })
}

fn without_group(cell: ElementRef<'_>, i: u8) -> Option<Column> {
    let html = cell.inner_html();
    let split = html.split(' ').collect::<Vec<&str>>();

    if split.len() == 1 {
        return Some(Column {
            subjects: vec![Subject {
                subject: split[0].to_string(),
                teacher: None,
                classroom: None,
            }],
            lesson_number: i,
        });
    }

    let subject = cell.select(&"span.p".to_sel()).next().map_or_else(
        || split.get(1).unwrap_or(&"").to_string(),
        |v| v.inner_html(),
    );

    let teacher = cell
        .select(&".n".to_sel())
        .next()
        .map_or_else(|| split[0].to_owned(), |v| v.inner_html());

    let classroom = cell.select(&"span.s".to_sel()).next().map_or_else(
        || {
            let vec = split;
            let len = vec.len();
            vec[len - 1].to_owned()
        },
        |v| v.inner_html(),
    );

    Some(Column {
        lesson_number: i,
        subjects: vec![Subject {
            subject: subject.to_owned(),
            teacher: Some(teacher),
            classroom: Some(classroom),
        }],
    })
}

fn lesson_cell(i: u8, cell: ElementRef<'_>, j: u8) -> Option<Column> {
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
            with_group(cell, j)
        } else {
            without_group(cell, i)
        }
    };

    lesson
}

/**
 * @route GET /plan/:id
 */
pub async fn plan(
    path: web::Path<String>,
    query: web::Query<Query>,
) -> Result<impl Responder, ApiError> {
    let id = if path.as_ref().chars().next().unwrap_or(' ').is_numeric() {
        get_id(&path.into_inner(), IdType::Class).await
    } else {
        get_id(&path.into_inner(), IdType::Teacher).await
    };

    let response = get_html(format!("http://www.zstrzeszow.pl/plan/plany/{id}.html")).await?;

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
                let lesson = lesson_cell(i, cell, j);

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

            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(json! { plan }))
        }
        Type::Row => {
            let plan = PlanRow {
                hours,
                weekdays: lessons_row,
            };

            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .json(json! { plan }))
        }
    };
}
