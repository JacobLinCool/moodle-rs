use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The competency framework id
    #[serde(rename = "competencyframeworkid")]
    pub r#competencyframeworkid: Option<i64>,
    /// A search string
    #[serde(rename = "search")]
    pub r#search: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFramework {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "framework")]
    pub r#framework: Option<ReturnsFramework>,
    /// True if this user has permission to manage competency frameworks
    #[serde(rename = "canmanage")]
    pub r#canmanage: Option<bool>,
    /// Context id for the framework
    #[serde(rename = "pagecontextid")]
    pub r#pagecontextid: Option<i64>,
    /// Current search string
    #[serde(rename = "search")]
    pub r#search: Option<String>,
    /// JSON encoded data for rules
    #[serde(rename = "rulesmodules")]
    pub r#rulesmodules: Option<String>,
    /// Plugin base url
    #[serde(rename = "pluginbaseurl")]
    pub r#pluginbaseurl: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_lp_data_for_competencies_manage_page", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_lp_data_for_competencies_manage_page", params)
        .await
}
