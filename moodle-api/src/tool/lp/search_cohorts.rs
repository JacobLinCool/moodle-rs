use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsContext {
    /// Context ID. Either use this value, or level and instanceid.
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// Context level. To be used with instanceid.
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// Context instance ID. To be used with level
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Query string
    #[serde(rename = "query")]
    pub r#query: Option<String>,
    #[serde(rename = "context")]
    pub r#context: Option<ParamsContext>,
    /// What other contexts to fetch the frameworks from. (all, parents, self)
    #[serde(rename = "includes")]
    pub r#includes: Option<String>,
    /// limitfrom we are fetching the records from
    #[serde(rename = "limitfrom")]
    pub r#limitfrom: Option<i64>,
    /// Number of records to fetch
    #[serde(rename = "limitnum")]
    pub r#limitnum: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCohortsItemCustomfieldsItem {
    /// The name of the custom field
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The shortname of the custom field - to be able to build the field class in the code
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// The type of the custom field - text field, checkbox...
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The raw value of the custom field
    #[serde(rename = "valueraw")]
    pub r#valueraw: Option<String>,
    /// The value of the custom field
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Custom fields
pub type r#ReturnsCohortsItemCustomfields = Vec<ReturnsCohortsItemCustomfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCohortsItem {
    /// ID of the cohort
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// cohort name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// cohort idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// cohort description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// cohort visible
    #[serde(rename = "visible")]
    pub r#visible: Option<bool>,
    /// cohort theme
    #[serde(rename = "theme")]
    pub r#theme: Option<String>,
    /// Custom fields
    #[serde(rename = "customfields")]
    pub r#customfields: Option<r#ReturnsCohortsItemCustomfields>,
}

pub type r#ReturnsCohorts = Vec<ReturnsCohortsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "cohorts")]
    pub r#cohorts: Option<r#ReturnsCohorts>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("tool_lp_search_cohorts", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("tool_lp_search_cohorts", params).await
}
