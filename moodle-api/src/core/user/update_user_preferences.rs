use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsPreferencesItem {
    /// The name of the preference
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The value of the preference, do not set this field if you want to remove (unset) the current value.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// User preferences
pub type r#ParamsPreferences = Vec<ParamsPreferencesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// id of the user, default to current user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Enable or disable notifications for this user
    #[serde(rename = "emailstop")]
    pub r#emailstop: Option<i64>,
    /// User preferences
    #[serde(rename = "preferences")]
    pub r#preferences: Option<r#ParamsPreferences>,
}
