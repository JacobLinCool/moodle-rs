use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsTemplate {
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
    /// The context id
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// The context level
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// The Instance id
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "template")]
    pub r#template: Option<ParamsTemplate>,
}
