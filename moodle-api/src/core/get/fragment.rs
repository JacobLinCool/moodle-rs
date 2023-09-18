use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsArgsItem {
    /// param name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// param value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// args for the callback are optional
pub type r#ParamsArgs = Vec<ParamsArgsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Component for the callback e.g. mod_assign
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// Name of the callback to execute
    #[serde(rename = "callback")]
    pub r#callback: Option<String>,
    /// Context ID that the fragment is from
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// args for the callback are optional
    #[serde(rename = "args")]
    pub r#args: Option<r#ParamsArgs>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// HTML fragment.
    #[serde(rename = "html")]
    pub r#html: Option<String>,
    /// JavaScript fragment
    #[serde(rename = "javascript")]
    pub r#javascript: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_get_fragment", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_get_fragment", params).await
}
