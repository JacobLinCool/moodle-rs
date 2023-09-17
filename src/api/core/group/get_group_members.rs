use serde::{self, Deserialize, Serialize};

pub type r#ParamsGroupids = Vec<Option<i64>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "groupids")]
    pub r#groupids: ParamsGroupids,
}

pub type r#ReturnsItemUserids = Vec<Option<i64>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// group record id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    #[serde(rename = "userids")]
    pub r#userids: ReturnsItemUserids,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_group_get_group_members", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
