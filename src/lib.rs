#[macro_use]
extern crate log;

use actix_web::http::header::ContentType;
use actix_web::{error, http::StatusCode, HttpResponse};
use entities::class::Class;
use entities::table::PlanRow;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

pub mod controllers;
pub mod entities;
pub mod utils;

#[derive(Debug, Clone)]
pub struct AppState {
    pub class_list: Vec<Class>,
    pub plan: Arc<Mutex<HashMap<String, PlanRow>>>,
}

#[derive(Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
}

/// Main `Err` enum linking multiple types
#[derive(Debug)]
pub enum ApiError {
    /// [`std`](std::io::Error) error type
    IoError(std::io::Error),
    /// [`serde_json`](serde_json::Error) error type
    SerdeError(serde_json::Error),
    /// [`reqwest`](reqwest::Error) error type
    ReqwestError(reqwest::Error),
    NoneValue(&'static str),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(v) => {
                warn!("Io error: {:?}", v);
                write!(f, "IoError")
            }
            Self::SerdeError(v) => {
                warn!("Error serializing: {:?}", v);
                write!(f, "SerdeError")
            }
            Self::ReqwestError(v) => {
                warn!("Request error: {:?}", v);
                write!(f, "ReqwestError")
            }
            Self::NoneValue(v) => {
                warn!("{:?} returned None", v);
                write!(f, "Value {v} is None")
            }
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(json!({ "message": self.to_string() }))
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::IoError(_) | Self::SerdeError(_) | Self::ReqwestError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::NoneValue(_) => StatusCode::NOT_FOUND,
        }
    }
}
