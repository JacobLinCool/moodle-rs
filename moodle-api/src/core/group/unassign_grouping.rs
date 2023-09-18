use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsUnassignmentsItem {
    /// grouping record id
    #[serde(rename = "groupingid")]
    pub r#groupingid: Option<i64>,
    /// group record id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
}

pub type r#ParamsUnassignments = Vec<ParamsUnassignmentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "unassignments")]
    pub r#unassignments: Option<r#ParamsUnassignments>,
}
