use serde::{self, Deserialize, Serialize};

/// List of event ids
pub type r#ParamsEventsEventids = Vec<i64>;

/// List of course ids for which events will be returned
pub type r#ParamsEventsCourseids = Vec<i64>;

/// List of group ids for which events should be returned
pub type r#ParamsEventsGroupids = Vec<i64>;

/// List of category ids for which events will be returned
pub type r#ParamsEventsCategoryids = Vec<i64>;

/// Event details
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsEvents {
    /// List of event ids
    #[serde(rename = "eventids")]
    pub r#eventids: Option<r#ParamsEventsEventids>,
    /// List of course ids for which events will be returned
    #[serde(rename = "courseids")]
    pub r#courseids: Option<r#ParamsEventsCourseids>,
    /// List of group ids for which events should be returned
    #[serde(rename = "groupids")]
    pub r#groupids: Option<r#ParamsEventsGroupids>,
    /// List of category ids for which events will be returned
    #[serde(rename = "categoryids")]
    pub r#categoryids: Option<r#ParamsEventsCategoryids>,
}

/// Options
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsOptions {
    /// Set to true to return current user's user events
    #[serde(rename = "userevents")]
    pub r#userevents: Option<bool>,
    /// Set to true to return site events
    #[serde(rename = "siteevents")]
    pub r#siteevents: Option<bool>,
    /// Time from which events should be returned
    #[serde(rename = "timestart")]
    pub r#timestart: Option<i64>,
    /// Time to which the events should be returned. We treat 0 and null as no end
    #[serde(rename = "timeend")]
    pub r#timeend: Option<i64>,
    /// Ignore hidden events or not
    #[serde(rename = "ignorehidden")]
    pub r#ignorehidden: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Event details
    #[serde(rename = "events")]
    pub r#events: Option<ParamsEvents>,
    /// Options
    #[serde(rename = "options")]
    pub r#options: Option<ParamsOptions>,
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
    /// Category id (only for category events).
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
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
        .post("core_calendar_get_calendar_events", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_calendar_get_calendar_events", params)
        .await
}
