use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// survey instance id
    #[serde(rename = "surveyid")]
    pub r#surveyid: Option<i64>,
}

/// Questions
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsQuestionsItem {
    /// Question id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Question text
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// Question short text
    #[serde(rename = "shorttext")]
    pub r#shorttext: Option<String>,
    /// Subquestions ids
    #[serde(rename = "multi")]
    pub r#multi: Option<String>,
    /// The question intro
    #[serde(rename = "intro")]
    pub r#intro: Option<String>,
    /// Question type
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    /// Question options
    #[serde(rename = "options")]
    pub r#options: Option<String>,
    /// Parent question (for subquestions)
    #[serde(rename = "parent")]
    pub r#parent: Option<i64>,
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
    let json = client.post("mod_survey_get_questions", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_survey_get_questions", params).await
}
