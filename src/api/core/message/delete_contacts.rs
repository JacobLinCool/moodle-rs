use serde::{self, Deserialize, Serialize};

/// List of user IDs
pub type r#ParamsUserids = Vec<Option<i64>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// List of user IDs
    #[serde(rename = "userids")]
    pub r#userids: ParamsUserids,
    /// The id of the user we are deleting the contacts for, 0 for the current user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}
