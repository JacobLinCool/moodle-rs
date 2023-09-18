use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Report ID
    #[serde(rename = "reportid")]
    pub r#reportid: Option<i64>,
    /// Whether editing mode is enabled
    #[serde(rename = "editmode")]
    pub r#editmode: Option<bool>,
    /// Page size
    #[serde(rename = "pagesize")]
    pub r#pagesize: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAttributesItem {
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// attributes
pub type r#ReturnsAttributes = Vec<ReturnsAttributesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSidebarmenucardsMenucardsItemItemsItem {
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// identifier
    #[serde(rename = "identifier")]
    pub r#identifier: Option<String>,
    /// title
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// action
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    /// disabled
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
}

/// items
pub type r#ReturnsSidebarmenucardsMenucardsItemItems =
    Vec<ReturnsSidebarmenucardsMenucardsItemItemsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSidebarmenucardsMenucardsItem {
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// key
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// items
    #[serde(rename = "items")]
    pub r#items: Option<r#ReturnsSidebarmenucardsMenucardsItemItems>,
}

/// menucards
pub type r#ReturnsSidebarmenucardsMenucards = Vec<ReturnsSidebarmenucardsMenucardsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSidebarmenucards {
    /// menucards
    #[serde(rename = "menucards")]
    pub r#menucards: Option<r#ReturnsSidebarmenucardsMenucards>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsConditionsAvailableconditionsItemOptiongroupValuesItem {
    /// value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// visiblename
    #[serde(rename = "visiblename")]
    pub r#visiblename: Option<String>,
}

/// values
pub type r#ReturnsConditionsAvailableconditionsItemOptiongroupValues =
    Vec<ReturnsConditionsAvailableconditionsItemOptiongroupValuesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsConditionsAvailableconditionsItemOptiongroup {
    /// text
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// values
    #[serde(rename = "values")]
    pub r#values: Option<r#ReturnsConditionsAvailableconditionsItemOptiongroupValues>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsConditionsAvailableconditionsItem {
    #[serde(rename = "optiongroup")]
    pub r#optiongroup: Option<ReturnsConditionsAvailableconditionsItemOptiongroup>,
}

/// availableconditions
pub type r#ReturnsConditionsAvailableconditions = Vec<ReturnsConditionsAvailableconditionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsConditions {
    /// hasavailableconditions
    #[serde(rename = "hasavailableconditions")]
    pub r#hasavailableconditions: Option<bool>,
    /// availableconditions
    #[serde(rename = "availableconditions")]
    pub r#availableconditions: Option<r#ReturnsConditionsAvailableconditions>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFiltersAvailablefiltersItemOptiongroupValuesItem {
    /// value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// visiblename
    #[serde(rename = "visiblename")]
    pub r#visiblename: Option<String>,
}

/// values
pub type r#ReturnsFiltersAvailablefiltersItemOptiongroupValues =
    Vec<ReturnsFiltersAvailablefiltersItemOptiongroupValuesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFiltersAvailablefiltersItemOptiongroup {
    /// text
    #[serde(rename = "text")]
    pub r#text: Option<String>,
    /// values
    #[serde(rename = "values")]
    pub r#values: Option<r#ReturnsFiltersAvailablefiltersItemOptiongroupValues>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFiltersAvailablefiltersItem {
    #[serde(rename = "optiongroup")]
    pub r#optiongroup: Option<ReturnsFiltersAvailablefiltersItemOptiongroup>,
}

/// availablefilters
pub type r#ReturnsFiltersAvailablefilters = Vec<ReturnsFiltersAvailablefiltersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFiltersActivefiltersItem {
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
pub type r#ReturnsFiltersActivefilters = Vec<ReturnsFiltersActivefiltersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFilters {
    /// hasavailablefilters
    #[serde(rename = "hasavailablefilters")]
    pub r#hasavailablefilters: Option<bool>,
    /// availablefilters
    #[serde(rename = "availablefilters")]
    pub r#availablefilters: Option<r#ReturnsFiltersAvailablefilters>,
    /// hasactivefilters
    #[serde(rename = "hasactivefilters")]
    pub r#hasactivefilters: Option<bool>,
    /// activefilters
    #[serde(rename = "activefilters")]
    pub r#activefilters: Option<r#ReturnsFiltersActivefilters>,
    /// helpicon
    #[serde(rename = "helpicon")]
    pub r#helpicon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSortingSortablecolumnsItemSorticon {
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
pub struct ReturnsSortingSortablecolumnsItem {
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
    pub r#sorticon: Option<ReturnsSortingSortablecolumnsItemSorticon>,
    /// movetitle
    #[serde(rename = "movetitle")]
    pub r#movetitle: Option<String>,
    /// sortenabledtitle
    #[serde(rename = "sortenabledtitle")]
    pub r#sortenabledtitle: Option<String>,
}

/// sortablecolumns
pub type r#ReturnsSortingSortablecolumns = Vec<ReturnsSortingSortablecolumnsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsSorting {
    /// hassortablecolumns
    #[serde(rename = "hassortablecolumns")]
    pub r#hassortablecolumns: Option<bool>,
    /// sortablecolumns
    #[serde(rename = "sortablecolumns")]
    pub r#sortablecolumns: Option<r#ReturnsSortingSortablecolumns>,
    /// helpicon
    #[serde(rename = "helpicon")]
    pub r#helpicon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCardview {
    /// form
    #[serde(rename = "form")]
    pub r#form: Option<String>,
    /// helpicon
    #[serde(rename = "helpicon")]
    pub r#helpicon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// source
    #[serde(rename = "source")]
    pub r#source: Option<String>,
    /// type
    #[serde(rename = "type")]
    pub r#type: Option<i64>,
    /// uniquerows
    #[serde(rename = "uniquerows")]
    pub r#uniquerows: Option<bool>,
    /// conditiondata
    #[serde(rename = "conditiondata")]
    pub r#conditiondata: Option<String>,
    /// settingsdata
    #[serde(rename = "settingsdata")]
    pub r#settingsdata: Option<String>,
    /// contextid
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// itemid
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// usercreated
    #[serde(rename = "usercreated")]
    pub r#usercreated: Option<i64>,
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// timecreated
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// timemodified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// usermodified
    #[serde(rename = "usermodified")]
    pub r#usermodified: Option<i64>,
    /// table
    #[serde(rename = "table")]
    pub r#table: Option<String>,
    /// filtersapplied
    #[serde(rename = "filtersapplied")]
    pub r#filtersapplied: Option<i64>,
    /// filterspresent
    #[serde(rename = "filterspresent")]
    pub r#filterspresent: Option<bool>,
    /// filtersform
    #[serde(rename = "filtersform")]
    pub r#filtersform: Option<String>,
    /// attributes
    #[serde(rename = "attributes")]
    pub r#attributes: Option<r#ReturnsAttributes>,
    /// classes
    #[serde(rename = "classes")]
    pub r#classes: Option<String>,
    /// editmode
    #[serde(rename = "editmode")]
    pub r#editmode: Option<bool>,
    #[serde(rename = "sidebarmenucards")]
    pub r#sidebarmenucards: Option<ReturnsSidebarmenucards>,
    #[serde(rename = "conditions")]
    pub r#conditions: Option<ReturnsConditions>,
    #[serde(rename = "filters")]
    pub r#filters: Option<ReturnsFilters>,
    #[serde(rename = "sorting")]
    pub r#sorting: Option<ReturnsSorting>,
    #[serde(rename = "cardview")]
    pub r#cardview: Option<ReturnsCardview>,
    /// javascript
    #[serde(rename = "javascript")]
    pub r#javascript: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_reportbuilder_reports_get", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_reportbuilder_reports_get", params).await
}
