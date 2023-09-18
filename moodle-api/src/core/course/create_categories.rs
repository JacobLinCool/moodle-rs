use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCategoriesItem {
    /// new category name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// the parent category id inside which the new category will be created - set to 0 for a root category
    #[serde(rename = "parent")]
    pub r#parent: Option<i64>,
    /// the new category idnumber
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// the new category description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// the new category theme. This option must be enabled on moodle
    #[serde(rename = "theme")]
    pub r#theme: Option<String>,
}

pub type r#ParamsCategories = Vec<ParamsCategoriesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "categories")]
    pub r#categories: Option<r#ParamsCategories>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsItem {
    /// new category id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// new category name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

pub type r#Returns = Vec<ReturnsItem>;

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_course_create_categories", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_course_create_categories", params).await
}
