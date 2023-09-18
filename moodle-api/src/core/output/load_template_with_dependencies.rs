use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// component containing the template
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// name of the template
    #[serde(rename = "template")]
    pub r#template: Option<String>,
    /// The current theme.
    #[serde(rename = "themename")]
    pub r#themename: Option<String>,
    /// Include comments or not
    #[serde(rename = "includecomments")]
    pub r#includecomments: Option<bool>,
    /// lang
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTemplatesItem {
    /// component containing the resource
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// name of the resource
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// resource value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

pub type r#ReturnsTemplates = Vec<ReturnsTemplatesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsStringsItem {
    /// component containing the resource
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// name of the resource
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// resource value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

pub type r#ReturnsStrings = Vec<ReturnsStringsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "templates")]
    pub r#templates: Option<r#ReturnsTemplates>,
    #[serde(rename = "strings")]
    pub r#strings: Option<r#ReturnsStrings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_output_load_template_with_dependencies", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_output_load_template_with_dependencies", params)
        .await
}
