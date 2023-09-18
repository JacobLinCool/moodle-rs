use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Data base record id for the course competency
    #[serde(rename = "coursecompetencyid")]
    pub r#coursecompetencyid: Option<i64>,
    /// Ruleoutcome value
    #[serde(rename = "ruleoutcome")]
    pub r#ruleoutcome: Option<i64>,
}
