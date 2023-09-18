use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// language
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
}
