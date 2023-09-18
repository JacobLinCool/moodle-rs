use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCategoriesItem {
    /// course id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// category name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// category id number
    #[serde(rename = "idnumber")]
    pub r#idnumber: Option<String>,
    /// parent category id
    #[serde(rename = "parent")]
    pub r#parent: Option<i64>,
    /// category description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// the category theme. This option must be enabled on moodle
    #[serde(rename = "theme")]
    pub r#theme: Option<String>,
}

pub type r#ParamsCategories = Vec<ParamsCategoriesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "categories")]
    pub r#categories: Option<r#ParamsCategories>,
}
