use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsSortdataItem {
    /// The name of a sortable column
    #[serde(rename = "sortby")]
    pub r#sortby: Option<String>,
    /// The direction that this column should be sorted by
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<String>,
}

/// The combined sort order of the table. Multiple fields can be specified.
pub type r#ParamsSortdata = Vec<ParamsSortdataItem>;

/// The value to filter on
pub type r#ParamsFiltersItemValues = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsFiltersItem {
    /// Name of the filter
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Type of join for filter values
    #[serde(rename = "jointype")]
    pub r#jointype: Option<i64>,
    /// The value to filter on
    #[serde(rename = "values")]
    pub r#values: Option<r#ParamsFiltersItemValues>,
}

/// The filters that will be applied in the request
pub type r#ParamsFilters = Vec<ParamsFiltersItem>;

pub type r#ParamsHiddencolumns = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// Handler
    #[serde(rename = "handler")]
    pub r#handler: Option<String>,
    /// Unique ID for the container
    #[serde(rename = "uniqueid")]
    pub r#uniqueid: Option<String>,
    /// The combined sort order of the table. Multiple fields can be specified.
    #[serde(rename = "sortdata")]
    pub r#sortdata: Option<r#ParamsSortdata>,
    /// The filters that will be applied in the request
    #[serde(rename = "filters")]
    pub r#filters: Option<r#ParamsFilters>,
    /// Type of join to join all filters together
    #[serde(rename = "jointype")]
    pub r#jointype: Option<i64>,
    /// The first initial to sort filter on
    #[serde(rename = "firstinitial")]
    pub r#firstinitial: Option<String>,
    /// The last initial to sort filter on
    #[serde(rename = "lastinitial")]
    pub r#lastinitial: Option<String>,
    /// The page number
    #[serde(rename = "pagenumber")]
    pub r#pagenumber: Option<i64>,
    /// The number of records per page
    #[serde(rename = "pagesize")]
    pub r#pagesize: Option<i64>,
    #[serde(rename = "hiddencolumns")]
    pub r#hiddencolumns: Option<r#ParamsHiddencolumns>,
    /// Whether the table preferences should be reset
    #[serde(rename = "resetpreferences")]
    pub r#resetpreferences: Option<bool>,
}

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWarningsItem {
    /// item
    #[serde(rename = "item")]
    pub r#item: Option<String>,
    /// item id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// the warning code can be used by the client app to implement specific behaviour
    #[serde(rename = "warningcode")]
    pub r#warningcode: Option<String>,
    /// untranslated english message to explain the warning
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

/// list of warnings
pub type r#ReturnsWarnings = Vec<ReturnsWarningsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// The raw html of the requested table.
    #[serde(rename = "html")]
    pub r#html: Option<String>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_table_get_dynamic_table_content", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_table_get_dynamic_table_content", params)
        .await
}
