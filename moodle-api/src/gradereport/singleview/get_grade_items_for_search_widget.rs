use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Course Id
    #[serde(rename = "courseid")]
    pub r#courseid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsGradeitemsItem {
    /// ID of the grade item
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// The full name of the grade item
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

pub type r#ReturnsGradeitems = Vec<ReturnsGradeitemsItem>;

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
    #[serde(rename = "gradeitems")]
    pub r#gradeitems: Option<r#ReturnsGradeitems>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post(
            "gradereport_singleview_get_grade_items_for_search_widget",
            params,
        )
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post(
            "gradereport_singleview_get_grade_items_for_search_widget",
            params,
        )
        .await
}
