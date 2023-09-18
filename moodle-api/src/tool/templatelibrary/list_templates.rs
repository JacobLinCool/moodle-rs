use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The component to search
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// The search string
    #[serde(rename = "search")]
    pub r#search: Option<String>,
    /// The current theme
    #[serde(rename = "themename")]
    pub r#themename: Option<String>,
}

pub type r#Returns = Vec<String>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_templatelibrary_list_templates", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_templatelibrary_list_templates", params)
        .await
}
