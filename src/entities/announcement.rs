use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Annoucement {
    pub date: String,
    pub title: String,
}

impl Annoucement {
    pub fn format(&self) -> String {
        format!("{} {}", &self.date, &self.title)
    }
}