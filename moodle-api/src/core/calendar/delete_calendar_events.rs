use serde::{self, Deserialize, Serialize};

/// List of events to delete
#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsEventsItem {
    /// Event ID
    #[serde(rename = "eventid")]
    pub r#eventid: Option<i64>,
    /// Delete comeplete series if repeated event
    #[serde(rename = "repeat")]
    pub r#repeat: Option<bool>,
}

pub type r#ParamsEvents = Vec<ParamsEventsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "events")]
    pub r#events: Option<r#ParamsEvents>,
}
