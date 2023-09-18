use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// User enrolment ID
    #[serde(rename = "ueid")]
    pub r#ueid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsErrorsItem {
    /// The data that failed the validation
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// The error message
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

/// List of validation errors
pub type r#ReturnsErrors = Vec<ReturnsErrorsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// True if the user's enrolment was successfully updated
    #[serde(rename = "result")]
    pub r#result: Option<bool>,
    /// List of validation errors
    #[serde(rename = "errors")]
    pub r#errors: Option<r#ReturnsErrors>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_enrol_unenrol_user_enrolment", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_enrol_unenrol_user_enrolment", params)
        .await
}
