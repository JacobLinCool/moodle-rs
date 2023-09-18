use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsPreflightdataItem {
    /// data name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// data value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Preflight required data (like passwords)
pub type r#ParamsPreflightdata = Vec<ParamsPreflightdataItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// quiz instance id
    #[serde(rename = "quizid")]
    pub r#quizid: Option<i64>,
    /// Preflight required data (like passwords)
    #[serde(rename = "preflightdata")]
    pub r#preflightdata: Option<r#ParamsPreflightdata>,
    /// Whether to force a new attempt or not.
    #[serde(rename = "forcenew")]
    pub r#forcenew: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAttempt {
    /// Attempt id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Foreign key reference to the quiz that was attempted.
    #[serde(rename = "quiz")]
    pub r#quiz: Option<i64>,
    /// Foreign key reference to the user whose attempt this is.
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Sequentially numbers this students attempts at this quiz.
    #[serde(rename = "attempt")]
    pub r#attempt: Option<i64>,
    /// Foreign key reference to the question_usage that holds the details of the the question_attempts that make up this quiz attempt.
    #[serde(rename = "uniqueid")]
    pub r#uniqueid: Option<i64>,
    /// Attempt layout.
    #[serde(rename = "layout")]
    pub r#layout: Option<String>,
    /// Attempt current page.
    #[serde(rename = "currentpage")]
    pub r#currentpage: Option<i64>,
    /// Whether is a preview attempt or not.
    #[serde(rename = "preview")]
    pub r#preview: Option<i64>,
    /// The current state of the attempts. 'inprogress', 'overdue', 'finished' or 'abandoned'.
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// Time when the attempt was started.
    #[serde(rename = "timestart")]
    pub r#timestart: Option<i64>,
    /// Time when the attempt was submitted. 0 if the attempt has not been submitted yet.
    #[serde(rename = "timefinish")]
    pub r#timefinish: Option<i64>,
    /// Last modified time.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Last modified time via webservices.
    #[serde(rename = "timemodifiedoffline")]
    pub r#timemodifiedoffline: Option<i64>,
    /// Next time quiz cron should check attempt for state changes.  NULL means never check.
    #[serde(rename = "timecheckstate")]
    pub r#timecheckstate: Option<i64>,
    /// Total marks for this attempt.
    #[serde(rename = "sumgrades")]
    pub r#sumgrades: Option<f64>,
    /// Time when the student was notified that manual grading of their attempt was complete.
    #[serde(rename = "gradednotificationsenttime")]
    pub r#gradednotificationsenttime: Option<i64>,
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
    #[serde(rename = "attempt")]
    pub r#attempt: Option<ReturnsAttempt>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_quiz_start_attempt", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_quiz_start_attempt", params).await
}
