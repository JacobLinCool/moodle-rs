use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Course to check, empty for site.
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}

pub type r#ReturnsAllowedeventtypes = Vec<Option<String>>;

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
    #[serde(rename = "allowedeventtypes")]
    pub r#allowedeventtypes: ReturnsAllowedeventtypes,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: ReturnsWarnings,
}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_calendar_get_allowed_event_types", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
