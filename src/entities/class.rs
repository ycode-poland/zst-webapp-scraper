use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Teacher {
    pub index: String,
    pub initials: String,
    pub name: String,
}
