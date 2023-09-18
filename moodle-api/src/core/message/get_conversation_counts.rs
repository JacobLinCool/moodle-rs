use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// id of the user, 0 for current user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTypes {
    /// Total number of individual conversations
    #[serde(rename = "1")]
    pub r#n1: Option<i64>,
    /// Total number of group conversations
    #[serde(rename = "2")]
    pub r#n2: Option<i64>,
    /// Total number of self conversations
    #[serde(rename = "3")]
    pub r#n3: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Total number of favourite conversations
    #[serde(rename = "favourites")]
    pub r#favourites: Option<i64>,
    #[serde(rename = "types")]
    pub r#types: Option<ReturnsTypes>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_message_get_conversation_counts", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_message_get_conversation_counts", params)
        .await
}
