use serde::{self, Deserialize, Serialize};

pub type r#ParamsGroupingids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "groupingids")]
    pub r#groupingids: Option<r#ParamsGroupingids>,
}
