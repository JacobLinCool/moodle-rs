use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The context level
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<i64>,
    /// The default category for the given context level
    #[serde(rename = "category")]
    pub r#category: Option<i64>,
    /// The default purpose for the given context level
    #[serde(rename = "purpose")]
    pub r#purpose: Option<i64>,
    /// The plugin name of the activity
    #[serde(rename = "activity")]
    pub r#activity: Option<String>,
    /// Whether to override existing instances with the defaults
    #[serde(rename = "override")]
    pub r#override: Option<bool>,
}

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
    /// Whether the context defaults were successfully set or not
    #[serde(rename = "result")]
    pub r#result: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_dataprivacy_set_context_defaults", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_dataprivacy_set_context_defaults", params)
        .await
}
