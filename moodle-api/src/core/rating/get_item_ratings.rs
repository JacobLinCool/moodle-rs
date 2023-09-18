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
    /// sort order (firstname, rating or timemodified)
    #[serde(rename = "sort")]
    pub r#sort: Option<String>,
}

/// Rating
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsRatingsItem {
    /// rating id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// URL user picture
    #[serde(rename = "userpictureurl")]
    pub r#userpictureurl: Option<String>,
    /// user fullname
    #[serde(rename = "userfullname")]
    pub r#userfullname: Option<String>,
    /// rating on scale
    #[serde(rename = "rating")]
    pub r#rating: Option<String>,
    /// time modified (timestamp)
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
}

/// list of ratings
pub type r#ReturnsRatings = Vec<ReturnsRatingsItem>;

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
    /// list of ratings
    #[serde(rename = "ratings")]
    pub r#ratings: Option<r#ReturnsRatings>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_rating_get_item_ratings", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_rating_get_item_ratings", params).await
}
