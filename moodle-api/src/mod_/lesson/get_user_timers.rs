use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// lesson instance id
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// the user id (empty for current user)
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

/// The timers
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTimersItem {
    /// The attempt id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The lesson id
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// The user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// First access time for a new timer session
    #[serde(rename = "starttime")]
    pub r#starttime: Option<i64>,
    /// Last access time to the lesson during the timer session
    #[serde(rename = "lessontime")]
    pub r#lessontime: Option<i64>,
    /// If the lesson for this timer was completed
    #[serde(rename = "completed")]
    pub r#completed: Option<i64>,
    /// Last modified time via webservices.
    #[serde(rename = "timemodifiedoffline")]
    pub r#timemodifiedoffline: Option<i64>,
}

pub type r#ReturnsTimers = Vec<ReturnsTimersItem>;

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
    #[serde(rename = "timers")]
    pub r#timers: Option<r#ReturnsTimers>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_lesson_get_user_timers", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_lesson_get_user_timers", params).await
}
