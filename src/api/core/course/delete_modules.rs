use serde::{self, Deserialize, Serialize};

/// Array of course module IDs
pub type r#ParamsCmids = Vec<Option<i64>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Array of course module IDs
    #[serde(rename = "cmids")]
    pub r#cmids: ParamsCmids,
}
