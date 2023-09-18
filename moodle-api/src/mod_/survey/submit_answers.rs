use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAnswersItem {
    /// Answer key
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Answer value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

pub type r#ParamsAnswers = Vec<ParamsAnswersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Survey id
    #[serde(rename = "surveyid")]
    pub r#surveyid: Option<i64>,
    #[serde(rename = "answers")]
    pub r#answers: Option<r#ParamsAnswers>,
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
    /// status: true if success
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_survey_submit_answers", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_survey_submit_answers", params).await
}
