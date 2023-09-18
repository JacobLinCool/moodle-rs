use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// context level: course, module, user, etc...
    #[serde(rename = "contextlevel")]
    pub r#contextlevel: Option<String>,
    /// the instance id of item associated with the context level
    #[serde(rename = "instanceid")]
    pub r#instanceid: Option<i64>,
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// rating area
    #[serde(rename = "ratingarea")]
    pub r#ratingarea: Option<String>,
    /// associated id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// scale id
    #[serde(rename = "scaleid")]
    pub r#scaleid: Option<i64>,
    /// user rating
    #[serde(rename = "rating")]
    pub r#rating: Option<i64>,
    /// rated user id
    #[serde(rename = "rateduserid")]
    pub r#rateduserid: Option<i64>,
    /// agreggation method
    #[serde(rename = "aggregation")]
    pub r#aggregation: Option<i64>,
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
    /// Whether the rate was successfully created
    #[serde(rename = "success")]
    pub r#success: Option<bool>,
    /// New aggregate
    #[serde(rename = "aggregate")]
    pub r#aggregate: Option<String>,
    /// Ratings count
    #[serde(rename = "count")]
    pub r#count: Option<i64>,
    /// Rating item id
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_rating_add_rating", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_rating_add_rating", params).await
}
