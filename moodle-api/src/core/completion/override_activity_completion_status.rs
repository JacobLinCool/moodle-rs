use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// course module id
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// the new activity completion state
    #[serde(rename = "newstate")]
    pub r#newstate: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// The course module id
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// The user id to which the completion info belongs
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The current completion state.
    #[serde(rename = "state")]
    pub r#state: Option<i64>,
    /// time of completion
    #[serde(rename = "timecompleted")]
    pub r#timecompleted: Option<i64>,
    /// The user id who has overriden the status, or null
    #[serde(rename = "overrideby")]
    pub r#overrideby: Option<i64>,
    /// type of tracking: 0 means none, 1 manual, 2 automatic
    #[serde(rename = "tracking")]
    pub r#tracking: Option<i64>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post(
            "core_completion_override_activity_completion_status",
            params,
        )
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post(
            "core_completion_override_activity_completion_status",
            params,
        )
        .await
}
