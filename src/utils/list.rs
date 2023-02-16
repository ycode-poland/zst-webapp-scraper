use crate::utils::scraper::get_html;
use scraper::Html;
use crate::utils::scraper::Scraper;
use std::collections::HashMap;

#[derive(Debug)]
pub enum IdType {
	Class,
	Teacher,
}

impl IdType {
	pub fn to_int(&self) -> u8 {
		match self {
		    Self::Class => 2,
			Self::Teacher => 4,
		}
	}
}

pub async fn get_id(name: &String, id_type: IdType) -> String {
	let mut map: HashMap<String, String> = HashMap::new();
	let response = get_html(format!("http://www.zstrzeszow.pl/plan/lista.html")).await.unwrap();

	let document = Html::parse_document(&response);
    let selector = format!("ul:nth-child({}) > li", id_type.to_int()).to_sel();
    let plans = document.select(&selector);

	plans.zip(0..).for_each(|(item, _i)| {
		let value = item.select(&"a".to_sel()).next().unwrap();
		let url = value.value().attr("href").unwrap().replace("plany/", "").replace(".html", "");
		let name: String;
		match &id_type {
			IdType::Class => {
				name = value.inner_html();
			},
			IdType::Teacher => {
				let val = value.inner_html().chars().rev().take(3).collect::<String>().replace(")", "");
				name = val.chars().rev().collect();
			}
		}
		map.insert(name, url);
	});

    map.get(name).unwrap_or(&"None".to_owned()).clone()
}
