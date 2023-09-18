use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCategoriesItem {
    /// category id to delete
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// the parent category to move the contents to, if specified
    #[serde(rename = "newparent")]
    pub r#newparent: Option<i64>,
    /// 1: recursively delete all contents inside this category, 0 (default): move contents to newparent or current parent category (except if parent is root)
    #[serde(rename = "recursive")]
    pub r#recursive: Option<bool>,
}

pub type r#ParamsCategories = Vec<ParamsCategoriesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "categories")]
    pub r#categories: Option<r#ParamsCategories>,
}
