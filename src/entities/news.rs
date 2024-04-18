use serde::Serialize;

#[derive(Serialize)]
pub struct Date {
    pub month: String,
    pub day_name: String,
    pub day: String,
}

#[derive(Serialize)]
pub struct News {
    pub image: String,
    pub news_type: String,
    pub title: String,
    pub date: Date,
    pub description: String,
}
