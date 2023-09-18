use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Course ID
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// User ID
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsStatusesItemDetailsItemRulevalue {
    /// Completion status
    #[serde(rename = "status")]
    pub r#status: Option<i64>,
    /// Completion description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsStatusesItemDetailsItem {
    /// Rule name
    #[serde(rename = "rulename")]
    pub r#rulename: Option<String>,
    #[serde(rename = "rulevalue")]
    pub r#rulevalue: Option<ReturnsStatusesItemDetailsItemRulevalue>,
}

/// Completion status details
pub type r#ReturnsStatusesItemDetails = Vec<ReturnsStatusesItemDetailsItem>;

/// Activity
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsStatusesItem {
    /// course module ID
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// activity module name
    #[serde(rename = "modname")]
    pub r#modname: Option<String>,
    /// instance ID
    #[serde(rename = "instance")]
    pub r#instance: Option<i64>,
    /// Completion state value: 0 means incomplete, 1 complete, 2 complete pass, 3 complete fail
    #[serde(rename = "state")]
    pub r#state: Option<i64>,
    /// timestamp for completed activity
    #[serde(rename = "timecompleted")]
    pub r#timecompleted: Option<i64>,
    /// type of tracking: 0 means none, 1 manual, 2 automatic
    #[serde(rename = "tracking")]
    pub r#tracking: Option<i64>,
    /// The user id who has overriden the status, or null
    #[serde(rename = "overrideby")]
    pub r#overrideby: Option<i64>,
    /// Whether the completion status affects the availability of another activity.
    #[serde(rename = "valueused")]
    pub r#valueused: Option<bool>,
    /// Whether this activity module has completion enabled
    #[serde(rename = "hascompletion")]
    pub r#hascompletion: Option<bool>,
    /// Whether this activity module instance tracks completion automatically.
    #[serde(rename = "isautomatic")]
    pub r#isautomatic: Option<bool>,
    /// Whether completion is being tracked for this user.
    #[serde(rename = "istrackeduser")]
    pub r#istrackeduser: Option<bool>,
    /// Whether this activity is visible to the user.
    #[serde(rename = "uservisible")]
    pub r#uservisible: Option<bool>,
    /// Completion status details
    #[serde(rename = "details")]
    pub r#details: Option<r#ReturnsStatusesItemDetails>,
}

/// List of activities status
pub type r#ReturnsStatuses = Vec<ReturnsStatusesItem>;

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
    /// List of activities status
    #[serde(rename = "statuses")]
    pub r#statuses: Option<r#ReturnsStatuses>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_completion_get_activities_completion_status", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_completion_get_activities_completion_status", params)
        .await
}
