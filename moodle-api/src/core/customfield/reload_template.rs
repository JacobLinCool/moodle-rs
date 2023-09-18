use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// itemid
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCategoriesItemFieldsItem {
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
}

pub type r#ReturnsCategoriesItemFields = Vec<ReturnsCategoriesItemFieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCategoriesItem {
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// inplace editable name
    #[serde(rename = "nameeditable")]
    pub r#nameeditable: Option<String>,
    /// addfieldmenu
    #[serde(rename = "addfieldmenu")]
    pub r#addfieldmenu: Option<String>,
    #[serde(rename = "fields")]
    pub r#fields: Option<r#ReturnsCategoriesItemFields>,
}

pub type r#ReturnsCategories = Vec<ReturnsCategoriesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// component
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// area
    #[serde(rename = "area")]
    pub r#area: Option<String>,
    /// itemid
    #[serde(rename = "itemid")]
    pub r#itemid: Option<i64>,
    /// view has categories
    #[serde(rename = "usescategories")]
    pub r#usescategories: Option<bool>,
    #[serde(rename = "categories")]
    pub r#categories: Option<r#ReturnsCategories>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_customfield_reload_template", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_customfield_reload_template", params)
        .await
}
