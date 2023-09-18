use serde::{self, Deserialize, Serialize};

/// 1 or more assignment ids
pub type r#ParamsAssignmentids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// 1 or more assignment ids
    #[serde(rename = "assignmentids")]
    pub r#assignmentids: Option<r#ParamsAssignmentids>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentsItemUserflagsItem {
    /// user flag id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
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

pub type r#ReturnsAssignmentsItemUserflags = Vec<ReturnsAssignmentsItemUserflagsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentsItem {
    /// assignment id
    #[serde(rename = "assignmentid")]
    pub r#assignmentid: Option<i64>,
    #[serde(rename = "userflags")]
    pub r#userflags: Option<r#ReturnsAssignmentsItemUserflags>,
}

/// list of assign user flag information
pub type r#ReturnsAssignments = Vec<ReturnsAssignmentsItem>;

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWarningsItem {
    /// item is always 'assignment'
    #[serde(rename = "item")]
    pub r#item: Option<String>,
    /// when errorcode is 3 then itemid is an assignment id. When errorcode is 1, itemid is a course module id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// errorcode can be 3 (no user flags found) or 1 (no permission to get user flags)
    #[serde(rename = "warningcode")]
    pub r#warningcode: Option<String>,
    /// untranslated english message to explain the warning
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

/// list of warnings
pub type r#ReturnsWarnings = Vec<ReturnsWarningsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// list of assign user flag information
    #[serde(rename = "assignments")]
    pub r#assignments: Option<r#ReturnsAssignments>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_assign_get_user_flags", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_assign_get_user_flags", params).await
}
