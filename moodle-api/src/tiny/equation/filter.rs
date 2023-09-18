use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The context ID
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// The equation content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Filtered content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("tiny_equation_filter", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("tiny_equation_filter", params).await
}
