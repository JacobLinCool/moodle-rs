use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The course id to obtain results for.
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// ID
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// Number of errors.
    #[serde(rename = "numerrors")]
    pub r#numerrors: Option<i64>,
    /// Number of checks.
    #[serde(rename = "numchecks")]
    pub r#numchecks: Option<i64>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("block_accessreview_get_module_data", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("block_accessreview_get_module_data", params)
        .await
}
