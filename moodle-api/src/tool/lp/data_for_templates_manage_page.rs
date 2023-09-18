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
pub struct ReturnsTemplatesItem {
    /// shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// duedate
    #[serde(rename = "duedate")]
    pub r#duedate: Option<i64>,
    /// visible
    #[serde(rename = "visible")]
    pub r#visible: Option<bool>,
    /// contextid
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
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
    /// duedateformatted
    #[serde(rename = "duedateformatted")]
    pub r#duedateformatted: Option<String>,
    /// cohortscount
    #[serde(rename = "cohortscount")]
    pub r#cohortscount: Option<i64>,
    /// planscount
    #[serde(rename = "planscount")]
    pub r#planscount: Option<i64>,
    /// canmanage
    #[serde(rename = "canmanage")]
    pub r#canmanage: Option<bool>,
    /// canread
    #[serde(rename = "canread")]
    pub r#canread: Option<bool>,
    /// contextname
    #[serde(rename = "contextname")]
    pub r#contextname: Option<String>,
    /// contextnamenoprefix
    #[serde(rename = "contextnamenoprefix")]
    pub r#contextnamenoprefix: Option<String>,
}

pub type r#ReturnsTemplates = Vec<ReturnsTemplatesItem>;

pub type r#ReturnsNavigation = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "templates")]
    pub r#templates: Option<r#ReturnsTemplates>,
    /// Url to the tool_lp plugin folder on this Moodle site
    #[serde(rename = "pluginbaseurl")]
    pub r#pluginbaseurl: Option<String>,
    #[serde(rename = "navigation")]
    pub r#navigation: Option<r#ReturnsNavigation>,
    /// The page context id
    #[serde(rename = "pagecontextid")]
    pub r#pagecontextid: Option<i64>,
    /// Whether the user manage the templates
    #[serde(rename = "canmanage")]
    pub r#canmanage: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_lp_data_for_templates_manage_page", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_lp_data_for_templates_manage_page", params)
        .await
}
