use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Id of course (empty or 0 for all the courses where the user is enrolled).
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// Id of user (empty or 0 for current user).
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// returns only groups in the specified grouping
    #[serde(rename = "groupingid")]
    pub r#groupingid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGroupsItem {
    /// group record id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// group name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// group description text
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// id number
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// course id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}

pub type r#ReturnsGroups = Vec<ReturnsGroupsItem>;

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWarningsItem {
    /// item
    #[serde(rename = "item")]
    pub r#item: Option<String>,
    /// item id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// the warning code can be used by the client app to implement specific behaviour
    #[serde(rename = "warningcode")]
    pub r#warningcode: Option<String>,
    /// untranslated english message to explain the warning
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

/// list of warnings
pub type r#ReturnsWarnings = Vec<ReturnsWarningsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "groups")]
    pub r#groups: Option<r#ReturnsGroups>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_group_get_course_user_groups", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_group_get_course_user_groups", params)
        .await
}
