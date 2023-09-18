use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Report ID
    #[serde(rename = "reportid")]
    pub r#reportid: Option<i64>,
    /// Column ID
    #[serde(rename = "columnid")]
    pub r#columnid: Option<i64>,
    /// New column sort position
    #[serde(rename = "position")]
    pub r#position: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSortablecolumnsItemSorticon {
    /// key
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// title
    #[serde(rename = "title")]
    pub r#title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSortablecolumnsItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// title
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// heading
    #[serde(rename = "heading")]
    pub r#heading: Option<String>,
    /// sortdirection
    #[serde(rename = "sortdirection")]
    pub r#sortdirection: Option<i64>,
    /// sortenabled
    #[serde(rename = "sortenabled")]
    pub r#sortenabled: Option<bool>,
    /// sortorder
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    #[serde(rename = "sorticon")]
    pub r#sorticon: Option<ReturnsSortablecolumnsItemSorticon>,
    /// movetitle
    #[serde(rename = "movetitle")]
    pub r#movetitle: Option<String>,
    /// sortenabledtitle
    #[serde(rename = "sortenabledtitle")]
    pub r#sortenabledtitle: Option<String>,
}

/// sortablecolumns
pub type r#ReturnsSortablecolumns = Vec<ReturnsSortablecolumnsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// hassortablecolumns
    #[serde(rename = "hassortablecolumns")]
    pub r#hassortablecolumns: Option<bool>,
    /// sortablecolumns
    #[serde(rename = "sortablecolumns")]
    pub r#sortablecolumns: Option<r#ReturnsSortablecolumns>,
    /// helpicon
    #[serde(rename = "helpicon")]
    pub r#helpicon: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_reportbuilder_columns_sort_reorder", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_reportbuilder_columns_sort_reorder", params)
        .await
}
