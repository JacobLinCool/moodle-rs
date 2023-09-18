use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The favourite area (itemtype)
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// id of the activity or whatever
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// id of the activity or whatever
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The favourite area (itemtype)
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// If created or deleted
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_course_toggle_activity_recommendation", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_course_toggle_activity_recommendation", params)
        .await
}
