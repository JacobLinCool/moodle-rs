use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsInstancesItem {
    /// ID of the course with meta enrolment.
    #[serde(rename = "metacourseid")]
    pub r#metacourseid: Option<i64>,
    /// ID of the course where meta enrolment is linked to.
    #[serde(rename = "courseid")]
    pub r#courseid: Option<String>,
}

/// List of course meta enrolment instances to delete.
pub type r#ParamsInstances = Vec<ParamsInstancesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// List of course meta enrolment instances to delete.
    #[serde(rename = "instances")]
    pub r#instances: Option<r#ParamsInstances>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// ID of the course where meta enrolment is deleted.
    #[serde(rename = "metacourseid")]
    pub r#metacourseid: Option<i64>,
    /// ID of the course that was meta linked.
    #[serde(rename = "courseid")]
    pub r#courseid: Option<String>,
    /// True on success, false if meta link did not exist.
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
}

/// List of course meta enrolment instances that were deleted.
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("enrol_meta_delete_instances", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("enrol_meta_delete_instances", params).await
}
