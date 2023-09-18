use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCohortsItemCategorytype {
    /// the name of the field: id (numeric value of course category id) or idnumber (alphanumeric value of idnumber course category) or system (value ignored)
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// the value of the categorytype
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCohortsItemCustomfieldsItem {
    /// The shortname of the custom field
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// The value of the custom field
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Custom fields for the cohort
pub type r#ParamsCohortsItemCustomfields = Vec<ParamsCohortsItemCustomfieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCohortsItem {
    #[serde(rename = "categorytype")]
    pub r#categorytype: Option<ParamsCohortsItemCategorytype>,
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
    /// the cohort theme. The allowcohortthemes setting must be enabled on Moodle
    #[serde(rename = "theme")]
    pub r#theme: Option<String>,
    /// Custom fields for the cohort
    #[serde(rename = "customfields")]
    pub r#customfields: Option<r#ParamsCohortsItemCustomfields>,
}

pub type r#ParamsCohorts = Vec<ParamsCohortsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "cohorts")]
    pub r#cohorts: Option<r#ParamsCohorts>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// cohort id
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
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_cohort_create_cohorts", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_cohort_create_cohorts", params).await
}
