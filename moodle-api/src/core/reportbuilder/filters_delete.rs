use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Report ID
    #[serde(rename = "reportid")]
    pub r#reportid: Option<i64>,
    /// Filter ID
    #[serde(rename = "filterid")]
    pub r#filterid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAvailablefiltersItemOptiongroupValuesItem {
    /// value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// visiblename
    #[serde(rename = "visiblename")]
    pub r#visiblename: Option<String>,
}

/// values
pub type r#ReturnsAvailablefiltersItemOptiongroupValues =
    Vec<ReturnsAvailablefiltersItemOptiongroupValuesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAvailablefiltersItemOptiongroup {
    /// text
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// values
    #[serde(rename = "values")]
    pub r#values: Option<r#ReturnsAvailablefiltersItemOptiongroupValues>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAvailablefiltersItem {
    #[serde(rename = "optiongroup")]
    pub r#optiongroup: Option<ReturnsAvailablefiltersItemOptiongroup>,
}

/// availablefilters
pub type r#ReturnsAvailablefilters = Vec<ReturnsAvailablefiltersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsActivefiltersItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// heading
    #[serde(rename = "heading")]
    pub r#heading: Option<String>,
    /// headingeditable
    #[serde(rename = "headingeditable")]
    pub r#headingeditable: Option<String>,
    /// sortorder
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// movetitle
    #[serde(rename = "movetitle")]
    pub r#movetitle: Option<String>,
    /// entityname
    #[serde(rename = "entityname")]
    pub r#entityname: Option<String>,
}

/// activefilters
pub type r#ReturnsActivefilters = Vec<ReturnsActivefiltersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// hasavailablefilters
    #[serde(rename = "hasavailablefilters")]
    pub r#hasavailablefilters: Option<bool>,
    /// availablefilters
    #[serde(rename = "availablefilters")]
    pub r#availablefilters: Option<r#ReturnsAvailablefilters>,
    /// hasactivefilters
    #[serde(rename = "hasactivefilters")]
    pub r#hasactivefilters: Option<bool>,
    /// activefilters
    #[serde(rename = "activefilters")]
    pub r#activefilters: Option<r#ReturnsActivefilters>,
    /// helpicon
    #[serde(rename = "helpicon")]
    pub r#helpicon: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_reportbuilder_filters_delete", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_reportbuilder_filters_delete", params)
        .await
}
