use serde::{self, Deserialize, Serialize};

/// event
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsEventsItem {
    /// event name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
    /// course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// number of repeats
    #[serde(rename = "repeats")]
    pub r#repeats: Option<i64>,
    /// Event type
    #[serde(rename = "eventtype")]
    pub r#eventtype: Option<String>,
    /// timestart
    #[serde(rename = "timestart")]
    pub r#timestart: Option<i64>,
    /// time duration
    #[serde(rename = "timeduration")]
    pub r#timeduration: Option<i64>,
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: i64,
    /// sequence
    #[serde(rename = "sequence")]
    pub r#sequence: i64,
}

pub type r#ParamsEvents = Vec<ParamsEventsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "events")]
    pub r#events: Option<r#ParamsEvents>,
}

/// event
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsEventsItem {
    /// event id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// event name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "format")]
    pub r#format: Option<i64>,
    /// course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// repeat id
    #[serde(rename = "repeatid")]
    pub r#repeatid: Option<i64>,
    /// module name
    #[serde(rename = "modulename")]
    pub r#modulename: Option<String>,
    /// instance id
    #[serde(rename = "instance")]
    pub r#instance: Option<i64>,
    /// Event type
    #[serde(rename = "eventtype")]
    pub r#eventtype: Option<String>,
    /// timestart
    #[serde(rename = "timestart")]
    pub r#timestart: Option<i64>,
    /// time duration
    #[serde(rename = "timeduration")]
    pub r#timeduration: Option<i64>,
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// unique id of ical events
    #[serde(rename = "uuid")]
    pub r#uuid: Option<String>,
    /// sequence
    #[serde(rename = "sequence")]
    pub r#sequence: Option<i64>,
    /// time modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Subscription id
    #[serde(rename = "subscriptionid")]
    pub r#subscriptionid: Option<i64>,
}

pub type r#ReturnsEvents = Vec<ReturnsEventsItem>;

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
    #[serde(rename = "events")]
    pub r#events: Option<r#ReturnsEvents>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_calendar_create_calendar_events", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_calendar_create_calendar_events", params)
        .await
}
