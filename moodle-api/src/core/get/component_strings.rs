use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// lang
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// string id
    #[serde(rename = "stringid")]
    pub r#stringid: Option<String>,
    /// translated string
    #[serde(rename = "string")]
    pub r#string: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_get_component_strings", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_get_component_strings", params).await
}
