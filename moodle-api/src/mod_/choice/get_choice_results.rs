use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// choice instance id
    #[serde(rename = "choiceid")]
    pub r#choiceid: Option<i64>,
}

/// User responses
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsOptionsItemUserresponsesItem {
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// user full name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// profile user image url
    #[serde(rename = "profileimageurl")]
    pub r#profileimageurl: Option<String>,
    /// answer id
    #[serde(rename = "answerid")]
    pub r#answerid: Option<i64>,
    /// time of modification
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
}

pub type r#ReturnsOptionsItemUserresponses = Vec<ReturnsOptionsItemUserresponsesItem>;

/// Options
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsOptionsItem {
    /// choice instance id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// text of the choice
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// maximum number of answers
    #[serde(rename = "maxanswer")]
    pub r#maxanswer: Option<i64>,
    #[serde(rename = "userresponses")]
    pub r#userresponses: Option<r#ReturnsOptionsItemUserresponses>,
    /// number of users answers
    #[serde(rename = "numberofuser")]
    pub r#numberofuser: Option<i64>,
    /// percentage of users answers
    #[serde(rename = "percentageamount")]
    pub r#percentageamount: Option<f64>,
}

pub type r#ReturnsOptions = Vec<ReturnsOptionsItem>;

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
    #[serde(rename = "options")]
    pub r#options: Option<r#ReturnsOptions>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_choice_get_choice_results", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_choice_get_choice_results", params).await
}
