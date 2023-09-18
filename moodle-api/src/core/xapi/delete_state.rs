use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Component name
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// xAPI activity ID IRI
    #[serde(rename = "activityId")]
    pub r#activity_id: Option<String>,
    /// The xAPI agent json
    #[serde(rename = "agent")]
    pub r#agent: Option<String>,
    /// The xAPI state ID
    #[serde(rename = "stateId")]
    pub r#state_id: Option<String>,
    /// The xAPI registration UUID
    #[serde(rename = "registration")]
    pub r#registration: Option<String>,
}
