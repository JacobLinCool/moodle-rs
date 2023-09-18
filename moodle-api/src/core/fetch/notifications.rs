use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Context ID
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItemVariables {
    /// HTML content of the Notification
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// Extra classes to provide to the tmeplate
    #[serde(rename = "extraclasses")]
    pub r#extraclasses: Option<String>,
    /// Whether to announce
    #[serde(rename = "announce")]
    pub r#announce: Option<String>,
    /// Whether to close
    #[serde(rename = "closebutton")]
    pub r#closebutton: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// Name of the template
    #[serde(rename = "template")]
    pub r#template: Option<String>,
    #[serde(rename = "variables")]
    pub r#variables: Option<ReturnsItemVariables>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_fetch_notifications", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_fetch_notifications", params).await
}
