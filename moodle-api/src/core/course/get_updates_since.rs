use serde::{self, Deserialize, Serialize};

/// Check only for updates in these areas
pub type r#ParamsFilter = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Course id to check
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Check updates since this time stamp
    #[serde(rename = "since")]
    pub r#since: Option<i64>,
    /// Check only for updates in these areas
    #[serde(rename = "filter")]
    pub r#filter: Option<r#ParamsFilter>,
}

/// The ids of the items updated
pub type r#ReturnsInstancesItemUpdatesItemItemids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsInstancesItemUpdatesItem {
    /// Name of the area updated.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Last time was updated
    #[serde(rename = "timeupdated")]
    pub r#timeupdated: Option<i64>,
    /// The ids of the items updated
    #[serde(rename = "itemids")]
    pub r#itemids: Option<r#ReturnsInstancesItemUpdatesItemItemids>,
}

pub type r#ReturnsInstancesItemUpdates = Vec<ReturnsInstancesItemUpdatesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsInstancesItem {
    /// The context level
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// Instance id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    #[serde(rename = "updates")]
    pub r#updates: Option<r#ReturnsInstancesItemUpdates>,
}

pub type r#ReturnsInstances = Vec<ReturnsInstancesItem>;

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
    #[serde(rename = "instances")]
    pub r#instances: Option<r#ReturnsInstances>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_course_get_updates_since", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_course_get_updates_since", params).await
}
