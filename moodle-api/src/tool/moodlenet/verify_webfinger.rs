use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The profile url that the user has given us
    #[serde(rename = "profileurl")]
    pub r#profileurl: Option<String>,
    /// The course we are adding to
    #[serde(rename = "course")]
    pub r#course: Option<i64>,
    /// The section within the course we are adding to
    #[serde(rename = "section")]
    pub r#section: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Was the passed content a valid WebFinger?
    #[serde(rename = "result")]
    pub r#result: Option<bool>,
    /// Our message for the user
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// Domain to redirect the user to
    #[serde(rename = "domain")]
    pub r#domain: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_moodlenet_verify_webfinger", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("tool_moodlenet_verify_webfinger", params).await
}
