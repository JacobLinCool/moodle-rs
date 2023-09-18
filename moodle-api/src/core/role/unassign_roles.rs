use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsUnassignmentsItem {
    /// Role to assign to the user
    #[serde(rename = "roleid")]
    pub r#roleid: Option<i64>,
    /// The user that is going to be assigned
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The context to unassign the user role from
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// The context level to unassign the user role in +                                    (block, course, coursecat, system, user, module)
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// The Instance id of item where the role needs to be unassigned
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
}

pub type r#ParamsUnassignments = Vec<ParamsUnassignmentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "unassignments")]
    pub r#unassignments: Option<r#ParamsUnassignments>,
}
