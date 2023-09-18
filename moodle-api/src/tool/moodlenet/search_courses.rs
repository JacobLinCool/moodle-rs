use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// search value
    #[serde(rename = "searchvalue")]
    pub r#searchvalue: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItem {
    /// course id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// course full name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// is the course visible
    #[serde(rename = "hidden")]
    pub r#hidden: Option<i64>,
    /// Next step of import
    #[serde(rename = "viewurl")]
    pub r#viewurl: Option<String>,
    /// Category name
    #[serde(rename = "coursecategory")]
    pub r#coursecategory: Option<String>,
    /// course image
    #[serde(rename = "courseimage")]
    pub r#courseimage: Option<String>,
}

pub type r#ReturnsCourses = Vec<ReturnsCoursesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "courses")]
    pub r#courses: Option<r#ReturnsCourses>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("tool_moodlenet_search_courses", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("tool_moodlenet_search_courses", params).await
}
