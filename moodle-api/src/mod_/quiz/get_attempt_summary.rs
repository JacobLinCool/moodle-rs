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
    /// Preflight required data (like passwords)
    #[serde(rename = "preflightdata")]
    pub r#preflightdata: Option<r#ParamsPreflightdata>,
}

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
    let json = client.post("mod_quiz_get_attempt_summary", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_quiz_get_attempt_summary", params).await
}
