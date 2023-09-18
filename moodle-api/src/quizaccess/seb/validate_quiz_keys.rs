use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Course module ID
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// Page URL to check
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// SEB config key
    #[serde(rename = "configkey")]
    pub r#configkey: Option<String>,
    /// SEB browser exam key
    #[serde(rename = "browserexamkey")]
    pub r#browserexamkey: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Is a provided config key valid?
    #[serde(rename = "configkey")]
    pub r#configkey: Option<bool>,
    /// Is a provided browser exam key valid?
    #[serde(rename = "browserexamkey")]
    pub r#browserexamkey: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("quizaccess_seb_validate_quiz_keys", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("quizaccess_seb_validate_quiz_keys", params)
        .await
}
