use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsContextsItem {
    /// The context level where the filters are: (coursecat, course, module)
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// The instance id of item associated with the context.
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
}

/// The list of contexts to check.
pub type r#ParamsContexts = Vec<ParamsContextsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The list of contexts to check.
    #[serde(rename = "contexts")]
    pub r#contexts: Option<r#ParamsContexts>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFiltersItem {
    /// The context level where the filters are: (coursecat, course, module).
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// The instance id of item associated with the context.
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
    /// The context id.
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// Filter plugin name.
    #[serde(rename = "filter")]
    pub r#filter: Option<String>,
    /// Filter state: 1 for on, -1 for off, 0 if inherit.
    #[serde(rename = "localstate")]
    pub r#localstate: Option<i64>,
    /// 1 or 0 to use when localstate is set to inherit.
    #[serde(rename = "inheritedstate")]
    pub r#inheritedstate: Option<i64>,
}

/// Available filters
pub type r#ReturnsFilters = Vec<ReturnsFiltersItem>;

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
    /// Available filters
    #[serde(rename = "filters")]
    pub r#filters: Option<r#ReturnsFilters>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_filters_get_available_in_context", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_filters_get_available_in_context", params)
        .await
}
