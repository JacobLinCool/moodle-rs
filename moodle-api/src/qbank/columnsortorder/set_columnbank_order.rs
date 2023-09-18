use serde::{self, Deserialize, Serialize};

pub type r#ParamsColumns = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "columns")]
    pub r#columns: Option<r#ParamsColumns>,
}
