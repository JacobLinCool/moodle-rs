use serde::{self, Deserialize, Serialize};

pub type r#ParamsCohortids = Vec<Option<i64>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "cohortids")]
    pub r#cohortids: ParamsCohortids,
}

pub type r#ReturnsItemUserids = Vec<Option<i64>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// cohort record id
    #[serde(rename = "cohortid")]
    pub r#cohortid: Option<i64>,
    #[serde(rename = "userids")]
    pub r#userids: ReturnsItemUserids,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_cohort_get_cohort_members", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
