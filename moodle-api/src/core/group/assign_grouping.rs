use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsAssignmentsItem {
    /// grouping record id
    #[serde(rename = "groupingid")]
    pub r#groupingid: Option<i64>,
    /// group record id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
}

pub type r#ParamsAssignments = Vec<ParamsAssignmentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "assignments")]
    pub r#assignments: Option<r#ParamsAssignments>,
}
