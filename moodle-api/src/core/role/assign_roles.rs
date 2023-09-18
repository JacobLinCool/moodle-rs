use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAssignmentsItem {
    /// Role to assign to the user
    #[serde(rename = "roleid")]
    pub r#roleid: Option<i64>,
    /// The user that is going to be assigned
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The context to assign the user role in
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// The context level to assign the user role in (block, course, coursecat, system, user, module)
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// The Instance id of item where the role needs to be assigned
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
}

pub type r#ParamsAssignments = Vec<ParamsAssignmentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "assignments")]
    pub r#assignments: Option<r#ParamsAssignments>,
}
