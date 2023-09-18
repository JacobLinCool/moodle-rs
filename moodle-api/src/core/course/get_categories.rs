use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCriteriaItem {
    /// The category column to search, expected keys (value format) are:"id" (int) the category id,"ids" (string) category ids separated by commas,"name" (string) the category name,"parent" (int) the parent category id,"idnumber" (string) category idnumber - user must have 'moodle/category:manage' to search on idnumber,"visible" (int) whether the returned categories must be visible or hidden. If the key is not passed, then the function return all categories that the user can see. - user must have 'moodle/category:manage' or 'moodle/category:viewhiddencategories' to search on visible,"theme" (string) only return the categories having this theme - user must have 'moodle/category:manage' to search on theme
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// the value to match
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// criteria
pub type r#ParamsCriteria = Vec<ParamsCriteriaItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// criteria
    #[serde(rename = "criteria")]
    pub r#criteria: Option<r#ParamsCriteria>,
    /// return the sub categories infos (1 - default) otherwise only the category info (0)
    #[serde(rename = "addsubcategories")]
    pub r#addsubcategories: Option<bool>,
}

/// List of categories
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// category id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// category name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// category id number
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// category description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// parent category id
    #[serde(rename = "parent")]
    pub r#parent: Option<i64>,
    /// category sorting order
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// number of courses in this category
    #[serde(rename = "coursecount")]
    pub r#coursecount: Option<i64>,
    /// 1: available, 0:not available
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// 1: available, 0:not available
    #[serde(rename = "visibleold")]
    pub r#visibleold: Option<i64>,
    /// timestamp
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// category depth
    #[serde(rename = "depth")]
    pub r#depth: Option<i64>,
    /// category path
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// category theme
    #[serde(rename = "theme")]
    pub r#theme: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_course_get_categories", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_course_get_categories", params).await
}
