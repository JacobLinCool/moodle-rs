use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Forum that the discussion is in
    #[serde(rename = "forumid")]
    pub r#forumid: Option<i64>,
    /// The discussion to lock / unlock
    #[serde(rename = "discussionid")]
    pub r#discussionid: Option<i64>,
    /// The timestamp for the lock state
    #[serde(rename = "targetstate")]
    pub r#targetstate: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTimes {
    /// The locked time of the discussion.
    #[serde(rename = "locked")]
    pub r#locked: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// The discussion we are locking.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The locked state of the discussion.
    #[serde(rename = "locked")]
    pub r#locked: Option<bool>,
    #[serde(rename = "times")]
    pub r#times: Option<ReturnsTimes>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_forum_set_lock_state", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_forum_set_lock_state", params).await
}
