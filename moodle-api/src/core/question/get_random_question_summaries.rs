use serde::{self, Deserialize, Serialize};

pub type r#ParamsTagids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Category id to find random questions
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
    /// Include the subcategories in the search
    #[serde(rename = "includesubcategories")]
    pub r#includesubcategories: Option<bool>,
    #[serde(rename = "tagids")]
    pub r#tagids: Option<r#ParamsTagids>,
    /// Context id that the questions will be rendered in (used for exporting)
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// Maximum number of results to return
    #[serde(rename = "limit")]
    pub r#limit: Option<i64>,
    /// Number of items to skip from the begging of the result set
    #[serde(rename = "offset")]
    pub r#offset: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsQuestionsItemIcon {
    /// key
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// alttext
    #[serde(rename = "alttext")]
    pub r#alttext: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsQuestionsItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// category
    #[serde(rename = "category")]
    pub r#category: Option<i64>,
    /// parent
    #[serde(rename = "parent")]
    pub r#parent: Option<i64>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// qtype
    #[serde(rename = "qtype")]
    pub r#qtype: Option<String>,
    #[serde(rename = "icon")]
    pub r#icon: Option<ReturnsQuestionsItemIcon>,
}

pub type r#ReturnsQuestions = Vec<ReturnsQuestionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// total number of questions in result set
    #[serde(rename = "totalcount")]
    pub r#totalcount: Option<i64>,
    #[serde(rename = "questions")]
    pub r#questions: Option<r#ReturnsQuestions>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_question_get_random_question_summaries", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_question_get_random_question_summaries", params)
        .await
}
