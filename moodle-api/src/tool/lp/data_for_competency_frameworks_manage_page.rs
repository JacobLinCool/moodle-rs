use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsPagecontext {
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
    #[serde(rename = "pagecontext")]
    pub r#pagecontext: Option<ParamsPagecontext>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCompetencyframeworksItem {
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
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<bool>,
    /// scaleid
    #[serde(rename = "scaleid")]
    pub r#scaleid: Option<i64>,
    /// scaleconfiguration
    #[serde(rename = "scaleconfiguration")]
    pub r#scaleconfiguration: Option<String>,
    /// contextid
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// taxonomies
    #[serde(rename = "taxonomies")]
    pub r#taxonomies: Option<String>,
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
    /// canmanage
    #[serde(rename = "canmanage")]
    pub r#canmanage: Option<bool>,
    /// competenciescount
    #[serde(rename = "competenciescount")]
    pub r#competenciescount: Option<i64>,
    /// contextname
    #[serde(rename = "contextname")]
    pub r#contextname: Option<String>,
    /// contextnamenoprefix
    #[serde(rename = "contextnamenoprefix")]
    pub r#contextnamenoprefix: Option<String>,
}

pub type r#ReturnsCompetencyframeworks = Vec<ReturnsCompetencyframeworksItem>;

pub type r#ReturnsNavigation = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "competencyframeworks")]
    pub r#competencyframeworks: Option<r#ReturnsCompetencyframeworks>,
    /// Url to the tool_lp plugin folder on this Moodle site
    #[serde(rename = "pluginbaseurl")]
    pub r#pluginbaseurl: Option<String>,
    #[serde(rename = "navigation")]
    pub r#navigation: Option<r#ReturnsNavigation>,
    /// The page context id
    #[serde(rename = "pagecontextid")]
    pub r#pagecontextid: Option<i64>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_lp_data_for_competency_frameworks_manage_page", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_lp_data_for_competency_frameworks_manage_page", params)
        .await
}
