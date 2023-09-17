use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The name of the plugin
    #[serde(rename = "plugin")]
    pub r#plugin: Option<String>,
    /// The target state
    #[serde(rename = "state")]
    pub r#state: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_admin_set_plugin_state", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
