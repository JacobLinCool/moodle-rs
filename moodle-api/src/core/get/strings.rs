use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsStringsItemStringparamsItem {
    /// param name - if the string expect only one $a parameter then don't send this field, just send the value.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// param value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// the definition of a string param (i.e. {$a->name})
pub type r#ParamsStringsItemStringparams = Vec<ParamsStringsItemStringparamsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsStringsItem {
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
    pub r#stringparams: Option<r#ParamsStringsItemStringparams>,
}

pub type r#ParamsStrings = Vec<ParamsStringsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "strings")]
    pub r#strings: Option<r#ParamsStrings>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// string id
    #[serde(rename = "stringid")]
    pub r#stringid: Option<String>,
    /// string component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// lang
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// translated string
    #[serde(rename = "string")]
    pub r#string: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_get_strings", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_get_strings", params).await
}
