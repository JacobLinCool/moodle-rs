use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsDataItem {
    /// key
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// year
    #[serde(rename = "year")]
    pub r#year: Option<i64>,
    /// month
    #[serde(rename = "month")]
    pub r#month: Option<i64>,
    /// day
    #[serde(rename = "day")]
    pub r#day: Option<i64>,
    /// hour
    #[serde(rename = "hour")]
    pub r#hour: Option<i64>,
    /// minute
    #[serde(rename = "minute")]
    pub r#minute: Option<i64>,
}

pub type r#ParamsData = Vec<ParamsDataItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "data")]
    pub r#data: Option<r#ParamsData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTimestampsItem {
    /// Timestamp key
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Unix timestamp
    #[serde(rename = "timestamp")]
    pub r#timestamp: Option<i64>,
}

pub type r#ReturnsTimestamps = Vec<ReturnsTimestampsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "timestamps")]
    pub r#timestamps: Option<r#ReturnsTimestamps>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_calendar_get_timestamps", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_calendar_get_timestamps", params).await
}
