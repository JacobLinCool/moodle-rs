use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// component containing the template
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// name of the template
    #[serde(rename = "template")]
    pub r#template: Option<String>,
}
