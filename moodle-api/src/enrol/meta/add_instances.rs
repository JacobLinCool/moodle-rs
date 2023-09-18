use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsInstancesItem {
    /// ID of the course where meta enrolment is added.
    #[serde(rename = "metacourseid")]
    pub r#metacourseid: Option<i64>,
    /// ID of the course where meta enrolment is linked to.
    #[serde(rename = "courseid")]
    pub r#courseid: Option<String>,
    /// Creates group in meta course named after linked course and puts all enrolled users in this group
    #[serde(rename = "creategroup")]
    pub r#creategroup: Option<bool>,
}

/// List of course meta enrolment instances to create.
pub type r#ParamsInstances = Vec<ParamsInstancesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// List of course meta enrolment instances to create.
    #[serde(rename = "instances")]
    pub r#instances: Option<r#ParamsInstances>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// ID of the course where meta enrolment is added.
    #[serde(rename = "metacourseid")]
    pub r#metacourseid: Option<i64>,
    /// ID of the course where meta enrolment is linked to.
    #[serde(rename = "courseid")]
    pub r#courseid: Option<String>,
    /// True on success, false if link already exists.
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
}

/// List of course meta enrolment instances that were created.
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("enrol_meta_add_instances", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("enrol_meta_add_instances", params).await
}
