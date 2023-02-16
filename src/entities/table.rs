use serde::Serialize;
use std::collections::HashMap;

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
pub struct PlanColumn {
    pub hours: Vec<String>,
    pub weekdays: HashMap<u8, Vec<Option<Column>>>,
}

#[derive(Debug, Serialize)]
pub struct PlanRow {
    pub hours: Vec<String>,
    pub weekdays: Vec<Vec<Option<Column>>>,
}
