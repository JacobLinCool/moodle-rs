use serde::{self, Deserialize, Serialize};

/// 1 or more user ids
pub type r#ParamsUserids = Vec<Option<i64>>;

/// 1 or more extension dates (timestamp)
pub type r#ParamsDates = Vec<Option<i64>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The assignment id to operate on
    #[serde(rename = "assignmentid")]
    pub r#assignmentid: Option<i64>,
    /// 1 or more user ids
    #[serde(rename = "userids")]
    pub r#userids: ParamsUserids,
    /// 1 or more extension dates (timestamp)
    #[serde(rename = "dates")]
    pub r#dates: ParamsDates,
}

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// item
    #[serde(rename = "item")]
    pub r#item: Option<String>,
    /// item id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// the warning code can be used by the client app to implement specific behaviour
    #[serde(rename = "warningcode")]
    pub r#warningcode: Option<String>,
    /// untranslated english message to explain the warning
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}

/// list of warnings
pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_assign_save_user_extensions", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}