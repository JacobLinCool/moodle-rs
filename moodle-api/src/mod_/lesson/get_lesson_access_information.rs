use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// lesson instance id
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
}

/// The reasons why the user cannot attempt the lesson
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreventaccessreasonsItem {
    /// Reason lang string code
    #[serde(rename = "reason")]
    pub r#reason: Option<String>,
    /// Additional data
    #[serde(rename = "data")]
    pub r#data: Option<String>,
    /// Complete html message
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

pub type r#ReturnsPreventaccessreasons = Vec<ReturnsPreventaccessreasonsItem>;

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
    /// Whether the user can manage the lesson or not.
    #[serde(rename = "canmanage")]
    pub r#canmanage: Option<bool>,
    /// Whether the user can grade the lesson or not.
    #[serde(rename = "cangrade")]
    pub r#cangrade: Option<bool>,
    /// Whether the user can view the lesson reports or not.
    #[serde(rename = "canviewreports")]
    pub r#canviewreports: Option<bool>,
    /// Whether the lesson is in review mode for the current user.
    #[serde(rename = "reviewmode")]
    pub r#reviewmode: Option<bool>,
    /// The number of attempts done by the user.
    #[serde(rename = "attemptscount")]
    pub r#attemptscount: Option<i64>,
    /// The last page seen id.
    #[serde(rename = "lastpageseen")]
    pub r#lastpageseen: Option<i64>,
    /// Whether the user left during a timed session.
    #[serde(rename = "leftduringtimedsession")]
    pub r#leftduringtimedsession: Option<bool>,
    /// The lesson first page id.
    #[serde(rename = "firstpageid")]
    pub r#firstpageid: Option<i64>,
    #[serde(rename = "preventaccessreasons")]
    pub r#preventaccessreasons: Option<r#ReturnsPreventaccessreasons>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_lesson_get_lesson_access_information", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_lesson_get_lesson_access_information", params)
        .await
}
