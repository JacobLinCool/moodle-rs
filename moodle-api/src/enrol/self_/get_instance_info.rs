use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// instance id of self enrolment plugin.
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// id of course enrolment instance
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// id of course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// type of enrolment plugin
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// name of enrolment plugin
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// status of enrolment plugin
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// password required for enrolment
    #[serde(rename = "enrolpassword")]
    pub r#enrolpassword: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("enrol_self_get_instance_info", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("enrol_self_get_instance_info", params).await
}
