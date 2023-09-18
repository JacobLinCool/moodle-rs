use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsGroupsItem {
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
    /// group visibility mode. 0 = Visible to all. 1 = Visible to members. 2 = See own membership. 3 = Membership is hidden. default: 0
    #[serde(rename = "visibility")]
    pub r#visibility: Option<i64>,
    /// activity participation enabled? Only for "all" and "members" visibility. Default true.
    #[serde(rename = "participation")]
    pub r#participation: Option<bool>,
}

/// List of group object. A group has a courseid, a name, a description and an enrolment key.
pub type r#ParamsGroups = Vec<ParamsGroupsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// List of group object. A group has a courseid, a name, a description and an enrolment key.
    #[serde(rename = "groups")]
    pub r#groups: Option<r#ParamsGroups>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
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
    /// group visibility mode. 0 = Visible to all. 1 = Visible to members. 2 = See own membership. 3 = Membership is hidden.
    #[serde(rename = "visibility")]
    pub r#visibility: Option<i64>,
    /// participation mode
    #[serde(rename = "participation")]
    pub r#participation: Option<bool>,
}

/// List of group object. A group has an id, a courseid, a name, a description and an enrolment key.
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_group_create_groups", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_group_create_groups", params).await
}
