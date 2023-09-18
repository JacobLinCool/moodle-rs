use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsGroupingsItem {
    /// id of grouping
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// multilang compatible name, course unique
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// grouping description text
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// id number
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
}

/// List of grouping object. A grouping has a courseid, a name and a description.
pub type r#ParamsGroupings = Vec<ParamsGroupingsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// List of grouping object. A grouping has a courseid, a name and a description.
    #[serde(rename = "groupings")]
    pub r#groupings: Option<r#ParamsGroupings>,
}
