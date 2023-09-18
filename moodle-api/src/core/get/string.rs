use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsStringparamsItem {
    /// param name - if the string expect only one $a parameter then don't send this field, just send the value.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// param value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// the definition of a string param (i.e. {$a->name})
pub type r#ParamsStringparams = Vec<ParamsStringparamsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// string identifier
    #[serde(rename = "stringid")]
    pub r#stringid: Option<String>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// lang
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// the definition of a string param (i.e. {$a->name})
    #[serde(rename = "stringparams")]
    pub r#stringparams: Option<r#ParamsStringparams>,
}
