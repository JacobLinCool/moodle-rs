use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsGroupsItem {
    /// ID of the group
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
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
    /// group visibility mode. 0 = Visible to all. 1 = Visible to members. 2 = See own membership. 3 = Membership is hidden.
    #[serde(rename = "visibility")]
    pub r#visibility: Option<String>,
    /// activity participation enabled? Only for "all" and "members" visibility
    #[serde(rename = "participation")]
    pub r#participation: Option<bool>,
}

/// List of group objects. A group is found by the id, then all other details provided will be updated.
pub type r#ParamsGroups = Vec<ParamsGroupsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// List of group objects. A group is found by the id, then all other details provided will be updated.
    #[serde(rename = "groups")]
    pub r#groups: Option<r#ParamsGroups>,
}
