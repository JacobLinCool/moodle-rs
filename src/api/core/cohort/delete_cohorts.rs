use serde::{self, Deserialize, Serialize};

pub type r#ParamsCohortids = Vec<Option<i64>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "cohortids")]
    pub r#cohortids: ParamsCohortids,
}
