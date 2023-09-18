use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsMembersItemCohorttype {
    /// The name of the field: id (numeric value of cohortid) or idnumber (alphanumeric value of idnumber)
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The value of the cohort
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsMembersItemUsertype {
    /// The name of the field: id (numeric value of id) or username (alphanumeric value of username)
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The value of the cohort
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsMembersItem {
    #[serde(rename = "cohorttype")]
    pub r#cohorttype: Option<ParamsMembersItemCohorttype>,
    #[serde(rename = "usertype")]
    pub r#usertype: Option<ParamsMembersItemUsertype>,
}

pub type r#ParamsMembers = Vec<ParamsMembersItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "members")]
    pub r#members: Option<r#ParamsMembers>,
}

/// warning
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsWarningsItem {
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
pub type r#ReturnsWarnings = Vec<ReturnsWarningsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_cohort_add_cohort_members", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_cohort_add_cohort_members", params).await
}
