pub mod plan_controller;

use actix_web::{
    HttpResponse, web
};

use scraper::Html;
use scraper::Selector;

use serde::Deserialize;
use serde_json::json;

use crate::entities::announcement::Annoucement;

#[derive(Deserialize)]
pub struct QueryStr {
    pub format: Option<bool>,
}


// GET /home
pub async fn home(query: web::Query<QueryStr>) -> HttpResponse {
    let response = reqwest::get(
        "https://zstrzeszow.pl/",
    )
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let document = scraper::Html::parse_document(&response);
    let selector_value = "#content_4 > ul > li";
    let announcement_selector = scraper::Selector::parse(&selector_value).unwrap();
    let titles = document.select(&announcement_selector).map(|x| x.inner_html());

    let mut announcements: Vec<Annoucement> = Vec::new();

    titles
        .zip(1..101)
        .for_each(|(item, number)| {
            let fragment = Html::parse_fragment(&item);
            let date_selector = Selector::parse("span").unwrap();
            let text_selector = Selector::parse("a").unwrap();

            let date = fragment.select(&date_selector).next().unwrap().inner_html();
            let text = fragment.select(&text_selector).next().unwrap().inner_html();

            announcements.push(
                Annoucement {
                    title: text,
                    date,
                }
            );
        });

    // let _ = &announcements.iter().for_each(|v| println!("{}", v.format()));

    if query.format.unwrap_or(false) {
        // let value = announcements.iter().for_each(|v| println!("{}", v.format()));
        let value: _ = announcements.iter().map(|v| format!("{}\n", v.format())).collect::<String>();
        // println!("{}", &value);
        return HttpResponse::Ok()
            .content_type("text/plain")
            .body(value);
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!{ announcements })
}