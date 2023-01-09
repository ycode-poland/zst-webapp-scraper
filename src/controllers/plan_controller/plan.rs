use std::collections::HashMap;

use actix_web::{HttpResponse, web};
use scraper::Html;
use serde_json::json;

use crate::{
    entities::table::{
        Column, Subject, Plan
    },
    utils::{
        class_to_id,
        scraper::{
            Scraper,
            get_html
        }
    }
};

/**
 * @route GET /plan/:id
 */
pub async fn plan(path: web::Path<String>) -> HttpResponse {

    let response = get_html(format!("http://www.zstrzeszow.pl/plan/plany/{}.html", class_to_id::parse(path.into_inner()))).await.unwrap();

    let document = Html::parse_document(&response);
    let plans_s = ".tabela > tbody > tr".to_sel();
    let plans = document.select(&plans_s);

    let mut hours: Vec<String> = Vec::new();
    let mut lessons: HashMap<u8, Vec<Option<Column>>> = HashMap::new();

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

            lesson_cells.zip(0..).for_each(|(cell, j)| {

                // If w zmiennej
                // Pozdrawiam tych co nie wierzyli :P
                let lesson: Option<Column> = if cell.inner_html() != "&nbsp;" {
                    let group_check = cell.select(&"br".to_sel()).next().map(|n| n.html()).unwrap_or_else(|| "".to_string());

                    if group_check == "<br>" {
                        let mut subjects: Vec<Subject> = Vec::new();

                        let mut subjects_tmp = Vec::new();
                        let mut teachers = Vec::new();
                        let mut classrooms = Vec::new();

                        cell.select(&"span.p".to_sel()).zip(0..).for_each(|(x, _num)| subjects_tmp.push(x.inner_html()));
                        cell.select(&"a.n".to_sel()).zip(0..).for_each(|(x, _num)| teachers.push(x.inner_html()));
                        cell.select(&"span.s".to_sel()).zip(0..).for_each(|(x, _num)| classrooms.push(x.inner_html()));

                        for (i, subject) in subjects_tmp.iter().enumerate() {
                            subjects.push(
                                Subject {
                                    subject: subject.clone(),
                                    teacher: teachers[i].clone(),
                                    classroom: classrooms[i].clone(),
                                }
                            )
                        }
                        Some(
                            Column {
                                lesson_number: j,
                                subjects,
                            }
                        )
                    } else {
                        let subject = cell.select(&"span.p".to_sel()).next().unwrap().inner_html();
                        let teacher = cell.select(&"a.n".to_sel()).next().unwrap().inner_html();
                        let classroom = cell.select(&"span.s".to_sel()).next().unwrap().inner_html();

                        Some(
                            Column {
                                lesson_number: j,
                                subjects: vec![
                                    Subject {
                                        subject,
                                        teacher,
                                        classroom,
                                    }
                                ],
                            }
                        )
                    }
                } else {
                    None
                };


                match j {
                    0 => monday.push(lesson),
                    1 => tuesday.push(lesson),
                    2 => wednesday.push(lesson),
                    3 => thursday.push(lesson),
                    4 => friday.push(lesson),
                    _ => {}
                }
            });

            hours.push(hour.inner_html().replace(' ', ""));
        }
    });

    lessons.insert(0, monday);
    lessons.insert(1, tuesday);
    lessons.insert(2, wednesday);
    lessons.insert(3, thursday);
    lessons.insert(4, friday);

    let plan = Plan {
        hours,
        weekdays: lessons,
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!{ plan })
}
