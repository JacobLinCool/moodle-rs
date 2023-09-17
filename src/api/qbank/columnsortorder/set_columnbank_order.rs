use serde::{self, Deserialize, Serialize};

pub type r#ParamsColumns = Vec<Option<String>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "columns")]
    pub r#columns: ParamsColumns,
}
