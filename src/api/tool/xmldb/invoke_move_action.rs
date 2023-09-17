use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Action
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    /// Plugin that is being edited
    #[serde(rename = "dir")]
    pub r#dir: Option<String>,
    /// Table name
    #[serde(rename = "table")]
    pub r#table: Option<String>,
    /// Field name
    #[serde(rename = "field")]
    pub r#field: Option<String>,
    /// Key name
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Index name
    #[serde(rename = "index")]
    pub r#index: Option<String>,
    /// How many positions to move by (negative - up, positive - down)
    #[serde(rename = "position")]
    pub r#position: Option<i64>,
}
