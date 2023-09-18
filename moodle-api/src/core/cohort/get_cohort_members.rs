use serde::{self, Deserialize, Serialize};

pub type r#ParamsCohortids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "cohortids")]
    pub r#cohortids: Option<r#ParamsCohortids>,
}

pub type r#ReturnsItemUserids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// cohort record id
    #[serde(rename = "cohortid")]
    pub r#cohortid: Option<i64>,
    #[serde(rename = "userids")]
    pub r#userids: Option<r#ReturnsItemUserids>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_cohort_get_cohort_members", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_cohort_get_cohort_members", params).await
}
