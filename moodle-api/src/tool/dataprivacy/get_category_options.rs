use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Include option "Inherit"
    #[serde(rename = "includeinherit")]
    pub r#includeinherit: Option<bool>,
    /// Include option "Not set"
    #[serde(rename = "includenotset")]
    pub r#includenotset: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsOptionsItem {
    /// The category ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The category name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

pub type r#ReturnsOptions = Vec<ReturnsOptionsItem>;

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
    #[serde(rename = "options")]
    pub r#options: Option<r#ReturnsOptions>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_dataprivacy_get_category_options", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_dataprivacy_get_category_options", params)
        .await
}
