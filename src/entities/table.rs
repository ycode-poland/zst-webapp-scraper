use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Subject {
    pub subject: String,
    pub teacher: String,
    pub classroom: String,
}

#[derive(Debug, Serialize)]
pub struct Column {
    pub subjects: Vec<Subject>,
    pub lesson_number: u8,
}

#[derive(Debug, Serialize)]
pub struct Plan {
    pub hours: Vec<String>,
    pub weekdays: HashMap<u8, Vec<Option<Column>>>
}