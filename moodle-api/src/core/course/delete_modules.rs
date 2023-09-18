use serde::{self, Deserialize, Serialize};

/// Array of course module IDs
pub type r#ParamsCmids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Array of course module IDs
    #[serde(rename = "cmids")]
    pub r#cmids: Option<r#ParamsCmids>,
}
