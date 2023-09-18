use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Report ID
    #[serde(rename = "reportid")]
    pub r#reportid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAvailableconditionsItemOptiongroupValuesItem {
    /// value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// visiblename
    #[serde(rename = "visiblename")]
    pub r#visiblename: Option<String>,
}

/// values
pub type r#ReturnsAvailableconditionsItemOptiongroupValues =
    Vec<ReturnsAvailableconditionsItemOptiongroupValuesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAvailableconditionsItemOptiongroup {
    /// text
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// values
    #[serde(rename = "values")]
    pub r#values: Option<r#ReturnsAvailableconditionsItemOptiongroupValues>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAvailableconditionsItem {
    #[serde(rename = "optiongroup")]
    pub r#optiongroup: Option<ReturnsAvailableconditionsItemOptiongroup>,
}

/// availableconditions
pub type r#ReturnsAvailableconditions = Vec<ReturnsAvailableconditionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// hasavailableconditions
    #[serde(rename = "hasavailableconditions")]
    pub r#hasavailableconditions: Option<bool>,
    /// availableconditions
    #[serde(rename = "availableconditions")]
    pub r#availableconditions: Option<r#ReturnsAvailableconditions>,
    /// hasactiveconditions
    #[serde(rename = "hasactiveconditions")]
    pub r#hasactiveconditions: Option<bool>,
    /// activeconditionsform
    #[serde(rename = "activeconditionsform")]
    pub r#activeconditionsform: Option<String>,
    /// helpicon
    #[serde(rename = "helpicon")]
    pub r#helpicon: Option<String>,
    /// javascript
    #[serde(rename = "javascript")]
    pub r#javascript: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_reportbuilder_conditions_reset", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_reportbuilder_conditions_reset", params)
        .await
}
