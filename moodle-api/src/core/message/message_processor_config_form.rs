use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsFormvaluesItem {
    /// name of the form element
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// value of the form element
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Config form values
pub type r#ParamsFormvalues = Vec<ParamsFormvaluesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// id of the user, 0 for current user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// The name of the message processor
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Config form values
    #[serde(rename = "formvalues")]
    pub r#formvalues: Option<r#ParamsFormvalues>,
}
