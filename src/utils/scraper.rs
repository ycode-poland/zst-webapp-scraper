use scraper::Selector;

pub async fn get_html(req: String) -> Result<String, reqwest::Error> {
    let response = reqwest::get(req).await?.text().await?;
    Ok(response)
}

pub trait Scraper {
    fn to_sel(&self) -> Selector;
}

impl Scraper for str {
    fn to_sel(&self) -> Selector {
        Selector::parse(self).unwrap()
    }
}
