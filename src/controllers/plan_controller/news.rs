use actix_web::HttpResponse;
use scraper::Html;
use serde_json::json;

use crate::{
    ApiError,
    entities::news::{News, Date},
    utils::scraper::{get_html, Scraper},
};

/**
 * @route GET /news
 */
pub async fn news() -> Result<HttpResponse, ApiError> {
    let response = get_html("http://www.zstrzeszow.pl".to_string()).await?;

    let document = scraper::Html::parse_document(&response);
    let news_s = "body > div.mainDataContainer div.newsDataContainer div.mainNewsPanel".to_sel();
    let news = document.select(&news_s).map(|x| x.inner_html());

    let mut news_struct: Vec<News> = Vec::new();

    news.zip(0..).for_each(|(item, _i)| {
        let fragment = Html::parse_fragment(&item);
        let image = fragment.select(&".newsTypePic img".to_sel()).next().unwrap().value().attr("src").unwrap().to_owned();

        let news_type_sel = ".newsCtn1 div > p".to_sel();
        let news_type = fragment.select(&news_type_sel).next().unwrap().inner_html();

        let title = fragment.select(&".newsCtn2 .newsTitle p".to_sel()).next().unwrap().inner_html();
        news_struct.push( News { image, news_type, title, date: Date { month: "".to_owned(), day_name: "".to_owned(), day: "".to_owned() }, description: "".to_owned() } );
    });

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(json! { news_struct }))
}
