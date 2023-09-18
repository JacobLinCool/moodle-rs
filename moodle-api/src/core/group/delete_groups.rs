use serde::{self, Deserialize, Serialize};

pub type r#ParamsGroupids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "groupids")]
    pub r#groupids: Option<r#ParamsGroupids>,
}
