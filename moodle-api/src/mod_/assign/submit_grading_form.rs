use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The assignment id to operate on
    #[serde(rename = "assignmentid")]
    pub r#assignmentid: Option<i64>,
    /// The user id the submission belongs to
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The data from the grading form, encoded as a json array
    #[serde(rename = "jsonformdata")]
    pub r#jsonformdata: Option<String>,
}

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
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
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_assign_submit_grading_form", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_assign_submit_grading_form", params).await
}
