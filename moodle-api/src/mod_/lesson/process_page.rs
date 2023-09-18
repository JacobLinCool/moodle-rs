use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsDataItem {
    /// data name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// data value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// the data to be saved
pub type r#ParamsData = Vec<ParamsDataItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// lesson instance id
    #[serde(rename = "lessonid")]
    pub r#lessonid: Option<i64>,
    /// the page id
    #[serde(rename = "pageid")]
    pub r#pageid: Option<i64>,
    /// the data to be saved
    #[serde(rename = "data")]
    pub r#data: Option<r#ParamsData>,
    /// optional password (the lesson may be protected)
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// if we want to review just after finishing (1 hour margin)
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
    /// New page id (if a jump was made).
    #[serde(rename = "newpageid")]
    pub r#newpageid: Option<i64>,
    /// Whether the page processing redirect directly to anoter page.
    #[serde(rename = "inmediatejump")]
    pub r#inmediatejump: Option<bool>,
    /// Whether there is not a default response.
    #[serde(rename = "nodefaultresponse")]
    pub r#nodefaultresponse: Option<bool>,
    /// The response feedback.
    #[serde(rename = "feedback")]
    pub r#feedback: Option<String>,
    /// Number of attempts remaining.
    #[serde(rename = "attemptsremaining")]
    pub r#attemptsremaining: Option<i64>,
    /// Whether the answer is correct.
    #[serde(rename = "correctanswer")]
    pub r#correctanswer: Option<bool>,
    /// Whether there aren't answers.
    #[serde(rename = "noanswer")]
    pub r#noanswer: Option<bool>,
    /// Whether is a essay question.
    #[serde(rename = "isessayquestion")]
    pub r#isessayquestion: Option<bool>,
    /// Whether we reachered the max number of attempts.
    #[serde(rename = "maxattemptsreached")]
    pub r#maxattemptsreached: Option<bool>,
    /// The response.
    #[serde(rename = "response")]
    pub r#response: Option<String>,
    /// The student answer.
    #[serde(rename = "studentanswer")]
    pub r#studentanswer: Option<String>,
    /// The user response.
    #[serde(rename = "userresponse")]
    pub r#userresponse: Option<String>,
    /// Whether the user is reviewing.
    #[serde(rename = "reviewmode")]
    pub r#reviewmode: Option<bool>,
    /// The ongoing message.
    #[serde(rename = "ongoingscore")]
    pub r#ongoingscore: Option<String>,
    /// Progress percentage in the lesson.
    #[serde(rename = "progress")]
    pub r#progress: Option<i64>,
    /// Whether we should display the menu or not in this page.
    #[serde(rename = "displaymenu")]
    pub r#displaymenu: Option<bool>,
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
    let json = client.post("mod_lesson_process_page", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_lesson_process_page", params).await
}
