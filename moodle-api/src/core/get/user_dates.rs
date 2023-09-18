use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsTimestampsItem {
    /// unix timestamp
    #[serde(rename = "timestamp")]
    pub r#timestamp: Option<i64>,
    /// format string
    #[serde(rename = "format")]
    pub r#format: Option<String>,
    /// The calendar type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Remove leading zero for day
    #[serde(rename = "fixday")]
    pub r#fixday: Option<i64>,
    /// Remove leading zero for hour
    #[serde(rename = "fixhour")]
    pub r#fixhour: Option<i64>,
}

pub type r#ParamsTimestamps = Vec<ParamsTimestampsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Context ID. Either use this value, or level and instanceid.
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// Context level. To be used with instanceid.
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// Context instance ID. To be used with level
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
    #[serde(rename = "timestamps")]
    pub r#timestamps: Option<r#ParamsTimestamps>,
}

pub type r#ReturnsDates = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "dates")]
    pub r#dates: Option<r#ReturnsDates>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_get_user_dates", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_get_user_dates", params).await
}
