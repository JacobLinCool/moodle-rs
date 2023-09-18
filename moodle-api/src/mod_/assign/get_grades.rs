use serde::{self, Deserialize, Serialize};

/// 1 or more assignment ids
pub type r#ParamsAssignmentids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// 1 or more assignment ids
    #[serde(rename = "assignmentids")]
    pub r#assignmentids: Option<r#ParamsAssignmentids>,
    /// timestamp, only return records where timemodified >= since
    #[serde(rename = "since")]
    pub r#since: Option<i64>,
}

/// grade information
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentsItemGradesItem {
    /// grade id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// assignment id
    #[serde(rename = "assignment")]
    pub r#assignment: Option<i64>,
    /// student id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// attempt number
    #[serde(rename = "attemptnumber")]
    pub r#attemptnumber: Option<i64>,
    /// grade creation time
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// grade last modified time
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// grader, -1 if grader is hidden
    #[serde(rename = "grader")]
    pub r#grader: Option<i64>,
    /// grade
    #[serde(rename = "grade")]
    pub r#grade: Option<String>,
    /// grade rendered into a format suitable for display
    #[serde(rename = "gradefordisplay")]
    pub r#gradefordisplay: Option<String>,
}

pub type r#ReturnsAssignmentsItemGrades = Vec<ReturnsAssignmentsItemGradesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAssignmentsItem {
    /// assignment id
    #[serde(rename = "assignmentid")]
    pub r#assignmentid: Option<i64>,
    #[serde(rename = "grades")]
    pub r#grades: Option<r#ReturnsAssignmentsItemGrades>,
}

/// list of assignment grade information
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
    /// errorcode can be 3 (no grades found) or 1 (no permission to get grades)
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
    /// list of assignment grade information
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
    let json = client.post("mod_assign_get_grades", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_assign_get_grades", params).await
}
