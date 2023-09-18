use serde::{self, Deserialize, Serialize};

/// Array of response ids
pub type r#ParamsResponses = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// choice instance id
    #[serde(rename = "choiceid")]
    pub r#choiceid: Option<i64>,
    /// Array of response ids
    #[serde(rename = "responses")]
    pub r#responses: Option<r#ParamsResponses>,
}

/// Answers
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAnswersItem {
    /// answer id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// choiceid
    #[serde(rename = "choiceid")]
    pub r#choiceid: Option<i64>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// optionid
    #[serde(rename = "optionid")]
    pub r#optionid: Option<i64>,
    /// time of last modification
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
}

pub type r#ReturnsAnswers = Vec<ReturnsAnswersItem>;

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
    #[serde(rename = "answers")]
    pub r#answers: Option<r#ReturnsAnswers>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_choice_submit_choice_response", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_choice_submit_choice_response", params)
        .await
}
