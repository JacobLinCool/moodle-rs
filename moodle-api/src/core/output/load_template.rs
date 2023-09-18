use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// component containing the template
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// name of the template
    #[serde(rename = "template")]
    pub r#template: Option<String>,
    /// The current theme.
    #[serde(rename = "themename")]
    pub r#themename: Option<String>,
    /// Include comments or not
    #[serde(rename = "includecomments")]
    pub r#includecomments: Option<bool>,
}
