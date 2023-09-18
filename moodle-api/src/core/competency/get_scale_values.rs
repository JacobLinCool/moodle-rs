use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The scale id
    #[serde(rename = "scaleid")]
    pub r#scaleid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// Scale value ID
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Scale value name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_competency_get_scale_values", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_competency_get_scale_values", params)
        .await
}
