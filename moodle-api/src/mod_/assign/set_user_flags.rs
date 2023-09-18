use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsUserflagsItem {
    /// student id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// locked
    #[serde(rename = "locked")]
    pub r#locked: Option<i64>,
    /// mailed
    #[serde(rename = "mailed")]
    pub r#mailed: Option<i64>,
    /// extension due date
    #[serde(rename = "extensionduedate")]
    pub r#extensionduedate: Option<i64>,
    /// marking workflow state
    #[serde(rename = "workflowstate")]
    pub r#workflowstate: Option<String>,
    /// allocated marker
    #[serde(rename = "allocatedmarker")]
    pub r#allocatedmarker: Option<i64>,
}

pub type r#ParamsUserflags = Vec<ParamsUserflagsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// assignment id
    #[serde(rename = "assignmentid")]
    pub r#assignmentid: Option<i64>,
    #[serde(rename = "userflags")]
    pub r#userflags: Option<r#ParamsUserflags>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// id of record if successful, -1 for failure
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// userid of record
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Failure error message
    #[serde(rename = "errormessage")]
    pub r#errormessage: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_assign_set_user_flags", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_assign_set_user_flags", params).await
}
