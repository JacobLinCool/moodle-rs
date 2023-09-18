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
    /// attempt id
    #[serde(rename = "attemptid")]
    pub r#attemptid: Option<i64>,
    /// page number
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// Preflight required data (like passwords)
    #[serde(rename = "preflightdata")]
    pub r#preflightdata: Option<r#ParamsPreflightdata>,
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

/// access messages, will only be returned for users with mod/quiz:preview capability,

/// for other users this method will throw an exception if there are messages
pub type r#ReturnsMessages = Vec<String>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsQuestionsItemResponsefileareasItemFilesItem {
    /// File name.
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    /// File path.
    #[serde(rename = "filepath")]
    pub r#filepath: Option<String>,
    /// File size.
    #[serde(rename = "filesize")]
    pub r#filesize: Option<i64>,
    /// Downloadable file url.
    #[serde(rename = "fileurl")]
    pub r#fileurl: Option<String>,
    /// Time modified.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// File mime type.
    #[serde(rename = "mimetype")]
    pub r#mimetype: Option<String>,
    /// Whether is an external file.
    #[serde(rename = "isexternalfile")]
    pub r#isexternalfile: Option<bool>,
    /// The repository type for external files.
    #[serde(rename = "repositorytype")]
    pub r#repositorytype: Option<String>,
}

/// Response files for the question
pub type r#ReturnsQuestionsItemResponsefileareasItemFiles =
    Vec<ReturnsQuestionsItemResponsefileareasItemFilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsQuestionsItemResponsefileareasItem {
    /// File area name
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// Response files for the question
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsQuestionsItemResponsefileareasItemFiles>,
}

/// Response file areas including files
pub type r#ReturnsQuestionsItemResponsefileareas = Vec<ReturnsQuestionsItemResponsefileareasItem>;

/// The question data. Some fields may not be returned depending on the quiz display settings.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsQuestionsItem {
    /// slot number
    #[serde(rename = "slot")]
    pub r#slot: Option<i64>,
    /// question type, i.e: multichoice
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// page of the quiz this question appears on
    #[serde(rename = "page")]
    pub r#page: Option<i64>,
    /// The question number to display for this question, e.g. "7", "i" or "Custom-B)".
    #[serde(rename = "questionnumber")]
    pub r#questionnumber: Option<String>,
    /// DO NOT USE. Use questionnumber. Only retained for backwards compatibility.
    #[serde(rename = "number")]
    pub r#number: Option<i64>,
    /// the question rendered
    #[serde(rename = "html")]
    pub r#html: Option<String>,
    /// Response file areas including files
    #[serde(rename = "responsefileareas")]
    pub r#responsefileareas: Option<r#ReturnsQuestionsItemResponsefileareas>,
    /// the number of real steps in this attempt
    #[serde(rename = "sequencecheck")]
    pub r#sequencecheck: Option<i64>,
    /// the timestamp of the most recent step in this question attempt
    #[serde(rename = "lastactiontime")]
    pub r#lastactiontime: Option<i64>,
    /// whether this question attempt has autosaved data
    #[serde(rename = "hasautosavedstep")]
    pub r#hasautosavedstep: Option<bool>,
    /// whether the question is flagged or not
    #[serde(rename = "flagged")]
    pub r#flagged: Option<bool>,
    /// the state where the question is in. It will not be returned if the user cannot see it due to the quiz display correctness settings.
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// current formatted state of the question
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// whether the question is blocked by the previous question
    #[serde(rename = "blockedbyprevious")]
    pub r#blockedbyprevious: Option<bool>,
    /// the mark awarded. It will be returned only if the user is allowed to see it.
    #[serde(rename = "mark")]
    pub r#mark: Option<String>,
    /// the maximum mark possible for this question attempt. It will be returned only if the user is allowed to see it.
    #[serde(rename = "maxmark")]
    pub r#maxmark: Option<f64>,
    /// Question settings (JSON encoded).
    #[serde(rename = "settings")]
    pub r#settings: Option<String>,
}

pub type r#ReturnsQuestions = Vec<ReturnsQuestionsItem>;

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
    /// access messages, will only be returned for users with mod/quiz:preview capability, for other users this method will throw an exception if there are messages
    #[serde(rename = "messages")]
    pub r#messages: Option<r#ReturnsMessages>,
    /// next page number
    #[serde(rename = "nextpage")]
    pub r#nextpage: Option<i64>,
    #[serde(rename = "questions")]
    pub r#questions: Option<r#ReturnsQuestions>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_quiz_get_attempt_data", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_quiz_get_attempt_data", params).await
}
