use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsSettings {
    /// New value of the setting
    #[serde(rename = "pushratingstouserplans")]
    pub r#pushratingstouserplans: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Course id for the course to update
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    #[serde(rename = "settings")]
    pub r#settings: Option<ParamsSettings>,
}
