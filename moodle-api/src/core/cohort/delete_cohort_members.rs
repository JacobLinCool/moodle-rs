use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsMembersItem {
    /// cohort record id
    #[serde(rename = "cohortid")]
    pub r#cohortid: Option<i64>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

pub type r#ParamsMembers = Vec<ParamsMembersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "members")]
    pub r#members: Option<r#ParamsMembers>,
}
