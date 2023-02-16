use actix_web::HttpResponse;
use scraper::Html;
use serde_json::json;

use crate::{
    entities::class::Class,
    utils::scraper::{get_html, Scraper},
};

/**
 * @route GET /plans
 */
pub async fn plans() -> HttpResponse {
    let response = get_html("http://www.zstrzeszow.pl/plan/lista.html".to_string())
        .await
        .unwrap();

    let document = scraper::Html::parse_document(&response);
    let plans_s = "body > ul:nth-child(2) > li".to_sel();
    let plans = document.select(&plans_s).map(|x| x.inner_html());

    let mut classes: Vec<Class> = Vec::new();

    plans.zip(1..30).for_each(|(item, _i)| {
        let fragment = Html::parse_fragment(&item);

        let fragment_select = fragment.select(&"a".to_sel()).next().unwrap();
        let index = fragment_select.value().attr("href").unwrap()[6..]
            .to_string()
            .replace(".html", "");
        let name = fragment_select.inner_html()[1..].to_string();
        let year = fragment_select.inner_html()[0..1]
            .to_string()
            .parse::<u8>()
            .unwrap();

        classes.push(Class { index, year, name });
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json! { classes })
}
