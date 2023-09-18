use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// lesson instance id
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// optional password (the lesson may be protected)
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// page id to continue from (only when continuing an attempt)
    #[serde(rename = "pageid")]
    pub r#pageid: Option<i64>,
    /// if we want to review just after finishing
    #[serde(rename = "review")]
    pub r#review: Option<bool>,
}

/// The lesson generated messages
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsMessagesItem {
    /// Message.
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// Message type: usually a CSS identifier like: success, info, warning, error, notifyproblem, notifyerror, notifytiny, notifysuccess
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

pub type r#ReturnsMessages = Vec<ReturnsMessagesItem>;

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
    #[serde(rename = "messages")]
    pub r#messages: Option<r#ReturnsMessages>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_lesson_launch_attempt", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_lesson_launch_attempt", params).await
}
