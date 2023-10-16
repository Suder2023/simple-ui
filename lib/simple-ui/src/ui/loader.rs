use crate::ui::*;
use serde_json::Value;
use std::path::PathBuf;

pub enum EUILoaderError {
    ReadJson(String),
    ParseJson(String),
    Unknow,
}

impl From<std::io::Error> for EUILoaderError {
    fn from(value: std::io::Error) -> Self {
        Self::ReadJson(format!("io-error: {}", value.to_string()))
    }
}

impl From<serde_json::Error> for EUILoaderError {
    fn from(value: serde_json::Error) -> Self {
        Self::ParseJson(format!("serde-error: {}", value.to_string()))
    }
}

pub fn loader(ui_file: &PathBuf) -> Result<Container, EUILoaderError> {
    let data = std::fs::read(ui_file)?;
    let _v: Value = serde_json::from_slice(&data)?;

    Err(EUILoaderError::Unknow)
}
