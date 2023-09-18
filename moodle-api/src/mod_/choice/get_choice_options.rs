use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// choice instance id
    #[serde(rename = "choiceid")]
    pub r#choiceid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsOptionsItem {
    /// option id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// text of the choice
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// maximum number of answers
    #[serde(rename = "maxanswers")]
    pub r#maxanswers: Option<i64>,
    /// true for orizontal, otherwise vertical
    #[serde(rename = "displaylayout")]
    pub r#displaylayout: Option<bool>,
    /// number of answers
    #[serde(rename = "countanswers")]
    pub r#countanswers: Option<i64>,
    /// we already answered
    #[serde(rename = "checked")]
    pub r#checked: Option<bool>,
    /// option disabled
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
}

/// Options
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
    /// Options
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
    let json = client.post("mod_choice_get_choice_options", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_choice_get_choice_options", params).await
}
