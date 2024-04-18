use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subject {
    pub subject: String,
    pub teacher: Option<String>,
    pub classroom: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Column {
    pub subjects: Vec<Subject>,
    pub lesson_number: u8,
}

#[derive(Debug, Serialize)]
pub struct PlanColumn {
    pub hours: Vec<String>,
    pub weekdays: HashMap<u8, Vec<Option<Column>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlanRow {
    pub hours: Vec<String>,
    pub weekdays: Vec<Vec<Option<Column>>>,
}
