use std::collections::HashMap;
use actix_web::{
    HttpResponse,
    web, get
};

use scraper::{ElementRef, Html};
use scraper::Selector;

use serde::Deserialize;
use serde_json::json;

use crate::entities::class::Class;
use crate::entities::table::{Column, Plan, Subject};

use crate::utils::class_to_id;

/**
 GET /plans
 Return {list of plans}
 */
pub async fn plans() -> HttpResponse {
    let response = reqwest::get("http://www.zstrzeszow.pl/plan/lista.html")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = scraper::Html::parse_document(&response);
    let selector_value = "body > ul:nth-child(2) > li";
    let plan_selector = scraper::Selector::parse(&selector_value).unwrap();
    let plans = document.select(&plan_selector).map(|x| x.inner_html());

    let mut classes: Vec<Class> = Vec::new();

    plans
        .zip(1..30)
        .for_each(|(item, number)| {
            let fragment = Html::parse_fragment(&item);
            let selector = Selector::parse("a").unwrap();

            let fragment_select = fragment.select(&selector).next().unwrap();
            let index = fragment_select.value().attr("href")
                                        .unwrap()[6..].to_string()
                                        .replace(".html", "");
            let name = fragment_select.inner_html()[1..].to_string();
            let year = fragment_select.inner_html()[0..1].to_string().parse::<u8>().unwrap();

            classes.push(
                Class {
                    index,
                    year,
                    name,
                }
            );
        });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!{ classes })
}

/**
 GET /plan/:id
 Return {given plan table}
 */
pub async fn plan(path: web::Path<String>) -> HttpResponse {
    let response = reqwest::get(
        format!("http://www.zstrzeszow.pl/plan/plany/{}.html", class_to_id::parse(path.into_inner())),
    )
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = Html::parse_document(&response);
    let selector_value = ".tabela > tbody > tr";
    let plan_selector = Selector::parse(&selector_value).unwrap();
    // let plans = document.select(&plan_selector).map(|x| x.inner_html());
    let plans = document.select(&plan_selector);

    let mut hours: Vec<String> = Vec::new();
    let mut lessons: HashMap<u8, Vec<Option<Column>>> = HashMap::new();

    let mut monday = Vec::new();
    let mut tuesday = Vec::new();
    let mut wednesday = Vec::new();
    let mut thursday = Vec::new();
    let mut friday = Vec::new();

    plans.zip(0..).for_each(|(item, i)| {
        if i > 0 {
            let hour_selector = Selector::parse("td.g").unwrap();
            let lesson_selector = Selector::parse("td.l").unwrap();
            let hour = item.select(&hour_selector).next().unwrap();
            let lesson_cells = item.select(&lesson_selector);

            lesson_cells.zip(0..).for_each(|(cell, j)| {

                let lesson: Option<Column> = if cell.inner_html() != "&nbsp;" {
                    let group_check_selector = Selector::parse("br").unwrap();
                    // let group_check: Result<ElementRef, String> = Ok(cell.select(&group_check_selector).next().unwrap_or(Err("".to_string())));
                    let group_check = cell.select(&group_check_selector).next().and_then(|n| Some(n.html())).unwrap_or("".to_string());

                    if group_check == "<br>".to_string() {
                        let subject_selector = Selector::parse("span.p").unwrap();
                        let teacher_selector = Selector::parse("a.n").unwrap();
                        let classroom_selector = Selector::parse("span.s").unwrap();

                        let mut subjects: Vec<Subject> = Vec::new();

                        let mut subjects_tmp = Vec::new();
                        let mut teachers = Vec::new();
                        let mut classrooms = Vec::new();

                        cell.select(&subject_selector).zip(0..).for_each(|(x, _num)| subjects_tmp.push(x.inner_html()));
                        cell.select(&teacher_selector).zip(0..).for_each(|(x, _num)| teachers.push(x.inner_html()));
                        cell.select(&classroom_selector).zip(0..).for_each(|(x, _num)| classrooms.push(x.inner_html()));

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
                        let subject_selector = Selector::parse("span.p").unwrap();
                        let subject = cell.select(&subject_selector).next().unwrap().inner_html();

                        let teacher_selector = Selector::parse("a.n").unwrap();
                        let teacher = cell.select(&teacher_selector).next().unwrap().inner_html();

                        let classroom_selector = Selector::parse("span.s").unwrap();
                        let classroom = cell.select(&classroom_selector).next().unwrap().inner_html();

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

            hours.push(hour.inner_html().replace(" ", ""));
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
