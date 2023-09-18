use serde::{self, Deserialize, Serialize};

/// Attempt ids
pub type r#ParamsAttemptids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// h5p activity instance id
    #[serde(rename = "h5pactivityid")]
    pub r#h5pactivityid: Option<i64>,
    /// Attempt ids
    #[serde(rename = "attemptids")]
    pub r#attemptids: Option<r#ParamsAttemptids>,
}

/// The option correct answer
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAttemptsItemResultsItemOptionsItemCorrectanswer {
    /// Option text value
    #[serde(rename = "answer")]
    pub r#answer: Option<String>,
    /// If has to be displayed as correct
    #[serde(rename = "correct")]
    pub r#correct: Option<bool>,
    /// If has to be displayed as incorrect
    #[serde(rename = "incorrect")]
    pub r#incorrect: Option<bool>,
    /// If has to be displayed as simple text
    #[serde(rename = "text")]
    pub r#text: Option<bool>,
    /// If has to be displayed as a checked option
    #[serde(rename = "checked")]
    pub r#checked: Option<bool>,
    /// If has to be displayed as a unchecked option
    #[serde(rename = "unchecked")]
    pub r#unchecked: Option<bool>,
    /// If has to be displayed as passed
    #[serde(rename = "pass")]
    pub r#pass: Option<bool>,
    /// If has to be displayed as failed
    #[serde(rename = "fail")]
    pub r#fail: Option<bool>,
}

/// The option user answer
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAttemptsItemResultsItemOptionsItemUseranswer {
    /// Option text value
    #[serde(rename = "answer")]
    pub r#answer: Option<String>,
    /// If has to be displayed as correct
    #[serde(rename = "correct")]
    pub r#correct: Option<bool>,
    /// If has to be displayed as incorrect
    #[serde(rename = "incorrect")]
    pub r#incorrect: Option<bool>,
    /// If has to be displayed as simple text
    #[serde(rename = "text")]
    pub r#text: Option<bool>,
    /// If has to be displayed as a checked option
    #[serde(rename = "checked")]
    pub r#checked: Option<bool>,
    /// If has to be displayed as a unchecked option
    #[serde(rename = "unchecked")]
    pub r#unchecked: Option<bool>,
    /// If has to be displayed as passed
    #[serde(rename = "pass")]
    pub r#pass: Option<bool>,
    /// If has to be displayed as failed
    #[serde(rename = "fail")]
    pub r#fail: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAttemptsItemResultsItemOptionsItem {
    /// Option description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Option string identifier
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The option correct answer
    #[serde(rename = "correctanswer")]
    pub r#correctanswer: Option<ReturnsAttemptsItemResultsItemOptionsItemCorrectanswer>,
    /// The option user answer
    #[serde(rename = "useranswer")]
    pub r#useranswer: Option<ReturnsAttemptsItemResultsItemOptionsItemUseranswer>,
}

/// The statement options
pub type r#ReturnsAttemptsItemResultsItemOptions = Vec<ReturnsAttemptsItemResultsItemOptionsItem>;

/// A single result statement tracking information
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAttemptsItemResultsItem {
    /// ID of the context
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// ID of the H5P attempt
    #[serde(rename = "attemptid")]
    pub r#attemptid: Option<i64>,
    /// Subcontent identifier
    #[serde(rename = "subcontent")]
    pub r#subcontent: Option<String>,
    /// Result creation
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Interaction type
    #[serde(rename = "interactiontype")]
    pub r#interactiontype: Option<String>,
    /// Result description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Result extra content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// Result score value
    #[serde(rename = "rawscore")]
    pub r#rawscore: Option<i64>,
    /// Result max score
    #[serde(rename = "maxscore")]
    pub r#maxscore: Option<i64>,
    /// Result duration in seconds
    #[serde(rename = "duration")]
    pub r#duration: Option<i64>,
    /// Result completion
    #[serde(rename = "completion")]
    pub r#completion: Option<i64>,
    /// Result success
    #[serde(rename = "success")]
    pub r#success: Option<i64>,
    /// Label used for result options
    #[serde(rename = "optionslabel")]
    pub r#optionslabel: Option<String>,
    /// Label used for correct answers
    #[serde(rename = "correctlabel")]
    pub r#correctlabel: Option<String>,
    /// Label used for user answers
    #[serde(rename = "answerlabel")]
    pub r#answerlabel: Option<String>,
    /// If the result has valid track information
    #[serde(rename = "track")]
    pub r#track: Option<bool>,
    /// The statement options
    #[serde(rename = "options")]
    pub r#options: Option<r#ReturnsAttemptsItemResultsItemOptions>,
}

/// The results of the attempt
pub type r#ReturnsAttemptsItemResults = Vec<ReturnsAttemptsItemResultsItem>;

/// The attempt general information
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAttemptsItem {
    /// ID of the context
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// ID of the H5P activity
    #[serde(rename = "h5pactivityid")]
    pub r#h5pactivityid: Option<i64>,
    /// ID of the user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Attempt creation
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Attempt modified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// Attempt number
    #[serde(rename = "attempt")]
    pub r#attempt: Option<i64>,
    /// Attempt score value
    #[serde(rename = "rawscore")]
    pub r#rawscore: Option<i64>,
    /// Attempt max score
    #[serde(rename = "maxscore")]
    pub r#maxscore: Option<i64>,
    /// Attempt duration in seconds
    #[serde(rename = "duration")]
    pub r#duration: Option<i64>,
    /// Attempt completion
    #[serde(rename = "completion")]
    pub r#completion: Option<i64>,
    /// Attempt success
    #[serde(rename = "success")]
    pub r#success: Option<i64>,
    /// Attempt scaled
    #[serde(rename = "scaled")]
    pub r#scaled: Option<f64>,
    /// The results of the attempt
    #[serde(rename = "results")]
    pub r#results: Option<r#ReturnsAttemptsItemResults>,
}

/// The complete attempts list
pub type r#ReturnsAttempts = Vec<ReturnsAttemptsItem>;

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

/// Activity attempts results data
#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Activity course module ID
    #[serde(rename = "activityid")]
    pub r#activityid: Option<i64>,
    /// The complete attempts list
    #[serde(rename = "attempts")]
    pub r#attempts: Option<r#ReturnsAttempts>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_h5pactivity_get_results", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_h5pactivity_get_results", params).await
}
