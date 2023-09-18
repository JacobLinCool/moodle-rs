use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {}

/// The list of Mobile addons this addon depends on.
pub type r#ReturnsPluginsItemDependencies = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPluginsItem {
    /// The plugin component name.
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// The plugin version number.
    #[serde(rename = "version")]
    pub r#version: Option<String>,
    /// The Mobile addon (package) name.
    #[serde(rename = "addon")]
    pub r#addon: Option<String>,
    /// The list of Mobile addons this addon depends on.
    #[serde(rename = "dependencies")]
    pub r#dependencies: Option<r#ReturnsPluginsItemDependencies>,
    /// The addon package url for download or empty if it doesn't exist.
    #[serde(rename = "fileurl")]
    pub r#fileurl: Option<String>,
    /// The addon package hash or empty if it doesn't exist.
    #[serde(rename = "filehash")]
    pub r#filehash: Option<String>,
    /// The addon package size or empty if it doesn't exist.
    #[serde(rename = "filesize")]
    pub r#filesize: Option<i64>,
    /// Handlers definition (JSON)
    #[serde(rename = "handlers")]
    pub r#handlers: Option<String>,
    /// Language strings used by the handlers (JSON)
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
}

pub type r#ReturnsPlugins = Vec<ReturnsPluginsItem>;

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWarningsItem {
    /// item
    #[serde(rename = "item")]
    pub r#item: Option<String>,
    /// item id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// the warning code can be used by the client app to implement specific behaviour
    #[serde(rename = "warningcode")]
    pub r#warningcode: Option<String>,
    /// untranslated english message to explain the warning
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

/// list of warnings
pub type r#ReturnsWarnings = Vec<ReturnsWarningsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "plugins")]
    pub r#plugins: Option<r#ReturnsPlugins>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_mobile_get_plugins_supporting_mobile", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_mobile_get_plugins_supporting_mobile", params)
        .await
}
