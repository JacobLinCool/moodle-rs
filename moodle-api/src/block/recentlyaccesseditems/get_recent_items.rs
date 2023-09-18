use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// result set limit
    #[serde(rename = "limit")]
    pub r#limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// courseid
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
    /// cmid
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// userid
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// modname
    #[serde(rename = "modname")]
    pub r#modname: Option<String>,
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// coursename
    #[serde(rename = "coursename")]
    pub r#coursename: Option<String>,
    /// timeaccess
    #[serde(rename = "timeaccess")]
    pub r#timeaccess: Option<i64>,
    /// viewurl
    #[serde(rename = "viewurl")]
    pub r#viewurl: Option<String>,
    /// courseviewurl
    #[serde(rename = "courseviewurl")]
    pub r#courseviewurl: Option<String>,
    /// icon
    #[serde(rename = "icon")]
    pub r#icon: Option<String>,
    /// purpose
    #[serde(rename = "purpose")]
    pub r#purpose: Option<String>,
}

/// The most recently accessed activities/resources by the logged user
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("block_recentlyaccesseditems_get_recent_items", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("block_recentlyaccesseditems_get_recent_items", params)
        .await
}
