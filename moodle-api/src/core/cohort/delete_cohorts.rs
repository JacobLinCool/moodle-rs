use serde::{self, Deserialize, Serialize};

pub type r#ParamsCohortids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "cohortids")]
    pub r#cohortids: Option<r#ParamsCohortids>,
}
