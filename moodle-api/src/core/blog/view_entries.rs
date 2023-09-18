use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsFiltersItem {
    /// The expected keys (value format) are: tag      PARAM_NOTAGS blog tag tagid    PARAM_INT    blog tag id userid   PARAM_INT    blog author (userid) cmid     PARAM_INT    course module id entryid  PARAM_INT    entry id groupid  PARAM_INT    group id courseid PARAM_INT    course id search   PARAM_RAW    search term
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The value of the filter.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Parameters used in the filter of view_entries.
pub type r#ParamsFilters = Vec<ParamsFiltersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Parameters used in the filter of view_entries.
    #[serde(rename = "filters")]
    pub r#filters: Option<r#ParamsFilters>,
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
    /// status: true if success
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_blog_view_entries", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_blog_view_entries", params).await
}
