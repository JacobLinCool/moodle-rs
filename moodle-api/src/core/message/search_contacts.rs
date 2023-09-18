use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// String the user's fullname has to match to be found
    #[serde(rename = "searchtext")]
    pub r#searchtext: Option<String>,
    /// Limit search to the user's courses
    #[serde(rename = "onlymycourses")]
    pub r#onlymycourses: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// User ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// User full name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// User picture URL
    #[serde(rename = "profileimageurl")]
    pub r#profileimageurl: Option<String>,
    /// Small user picture URL
    #[serde(rename = "profileimageurlsmall")]
    pub r#profileimageurlsmall: Option<String>,
}

/// List of contacts
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_message_search_contacts", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_message_search_contacts", params).await
}
