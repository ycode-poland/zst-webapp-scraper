use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Class {
    pub index: String,
    pub year: u8,
    pub name: String,
}

impl Class {
    pub fn format(&self) -> String {
        format!("{}. {}{}", &self.index, &self.year.to_string(), &self.name)
    }
}