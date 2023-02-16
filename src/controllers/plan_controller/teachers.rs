use actix_web::HttpResponse;
use scraper::Html;
use serde_json::json;

use crate::{
    entities::class::Teacher,
    utils::scraper::{
        Scraper,
        get_html
    },
	utils::list::IdType
};

/**
 * @route GET /teachers
 */
pub async fn teachers() -> HttpResponse {
    let response = get_html("http://www.zstrzeszow.pl/plan/lista.html".to_string()).await.unwrap();

    let document = scraper::Html::parse_document(&response);
    let plans_s = format!("ul:nth-child({}) > li", IdType::Teacher.to_int()).to_sel();
    let plans = document.select(&plans_s).map(|x| x.inner_html());

    let mut teachers: Vec<Teacher> = Vec::new();

    plans.zip(1..30)
        .for_each(|(item, _i)| {
            let fragment = Html::parse_fragment(&item);

            let fragment_select = fragment.select(&"a".to_sel()).next().unwrap();
            let index = fragment_select.value().attr("href").unwrap()[6..].to_string().replace(".html", "");
			let teacher_name_len = fragment_select.inner_html().len();
            let name = fragment_select.inner_html()[0..&teacher_name_len-5].to_string();
            let initials = fragment_select.inner_html()[&teacher_name_len-3..&teacher_name_len-1].to_string();

            teachers.push(
                Teacher {
                    index,
                    initials,
                    name,
                }
            );
        });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!{ teachers })
}
