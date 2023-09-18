use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsFiltersItem {
    /// Column name to filter by
    #[serde(rename = "column")]
    pub r#column: Option<String>,
    /// Value to filter by. Must be exact match
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

pub type r#ParamsFilters = Vec<ParamsFiltersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "filters")]
    pub r#filters: Option<r#ParamsFilters>,
    /// Column to sort by.
    #[serde(rename = "sort")]
    pub r#sort: Option<String>,
    /// Sort direction. Should be either ASC or DESC
    #[serde(rename = "order")]
    pub r#order: Option<String>,
    /// Skip this number of records before returning results
    #[serde(rename = "skip")]
    pub r#skip: Option<i64>,
    /// Return this number of records at most.
    #[serde(rename = "limit")]
    pub r#limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// sortorder
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// parentid
    #[serde(rename = "parentid")]
    pub r#parentid: Option<i64>,
    /// path
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// ruleoutcome
    #[serde(rename = "ruleoutcome")]
    pub r#ruleoutcome: Option<i64>,
    /// ruletype
    #[serde(rename = "ruletype")]
    pub r#ruletype: Option<String>,
    /// ruleconfig
    #[serde(rename = "ruleconfig")]
    pub r#ruleconfig: Option<String>,
    /// scaleid
    #[serde(rename = "scaleid")]
    pub r#scaleid: Option<i64>,
    /// scaleconfiguration
    #[serde(rename = "scaleconfiguration")]
    pub r#scaleconfiguration: Option<String>,
    /// competencyframeworkid
    #[serde(rename = "competencyframeworkid")]
    pub r#competencyframeworkid: Option<i64>,
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
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_competency_list_competencies", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_competency_list_competencies", params)
        .await
}
