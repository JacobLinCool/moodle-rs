use serde::{self, Deserialize, Serialize};

/// List of cohort id. A cohort id is an integer.
pub type r#ParamsCohortids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// List of cohort id. A cohort id is an integer.
    #[serde(rename = "cohortids")]
    pub r#cohortids: Option<r#ParamsCohortids>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemCustomfieldsItem {
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
pub type r#ReturnsItemCustomfields = Vec<ReturnsItemCustomfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
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
    pub r#customfields: Option<r#ReturnsItemCustomfields>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_cohort_get_cohorts", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_cohort_get_cohorts", params).await
}
