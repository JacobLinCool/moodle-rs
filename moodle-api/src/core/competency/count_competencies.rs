use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsFiltersItem {
    /// Column name to filter by
    #[serde(rename = "column")]
    pub r#column: Option<String>,
    /// Value to filter by. Must be exact match
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

pub type r#ParamsFilters = Vec<ParamsFiltersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "filters")]
    pub r#filters: Option<r#ParamsFilters>,
}
