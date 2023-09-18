use serde::{self, Deserialize, Serialize};

/// List of grouping id. A grouping id is an integer.
pub type r#ParamsGroupingids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// List of grouping id. A grouping id is an integer.
    #[serde(rename = "groupingids")]
    pub r#groupingids: Option<r#ParamsGroupingids>,
    /// return associated groups
    #[serde(rename = "returngroups")]
    pub r#returngroups: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemGroupsItem {
    /// group record id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// id of course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// multilang compatible name, course unique
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// group description text
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// group enrol secret phrase
    #[serde(rename = "enrolmentkey")]
    pub r#enrolmentkey: Option<String>,
    /// id number
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
}

/// optional groups
pub type r#ReturnsItemGroups = Vec<ReturnsItemGroupsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// grouping record id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// id of course
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
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
    /// optional groups
    #[serde(rename = "groups")]
    pub r#groups: Option<r#ReturnsItemGroups>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_group_get_groupings", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_group_get_groupings", params).await
}
