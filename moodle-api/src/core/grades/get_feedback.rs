use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Course ID
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// User ID
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Grade Item ID
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// The full feedback text
    #[serde(rename = "feedbacktext")]
    pub r#feedbacktext: Option<String>,
    /// Title of the grade item that the feedback is for
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Students name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// Students picture
    #[serde(rename = "picture")]
    pub r#picture: Option<String>,
    /// Additional field for the user (email or ID number, for example)
    #[serde(rename = "additionalfield")]
    pub r#additionalfield: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_grades_get_feedback", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_grades_get_feedback", params).await
}
