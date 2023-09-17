use serde::{self, Deserialize, Serialize};

pub type r#ParamsUserids = Vec<Option<i64>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "userids")]
    pub r#userids: ParamsUserids,
}
