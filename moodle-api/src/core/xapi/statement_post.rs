use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Component name
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// json object with all the statements to post
    #[serde(rename = "requestjson")]
    pub r#requestjson: Option<String>,
}

/// List of statements storing acceptance results
pub type r#Returns = Vec<bool>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_xapi_statement_post", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_xapi_statement_post", params).await
}
