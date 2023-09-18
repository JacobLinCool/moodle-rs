use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The context id that owns the editor
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// The page hash
    #[serde(rename = "pagehash")]
    pub r#pagehash: Option<String>,
    /// The page instance
    #[serde(rename = "pageinstance")]
    pub r#pageinstance: Option<String>,
    /// The ID of the element
    #[serde(rename = "elementid")]
    pub r#elementid: Option<String>,
    /// The new draft item id to resume files to
    #[serde(rename = "draftid")]
    pub r#draftid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// The draft text
    #[serde(rename = "drafttext")]
    pub r#drafttext: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("tiny_autosave_resume_session", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("tiny_autosave_resume_session", params).await
}
