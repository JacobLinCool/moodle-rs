use serde::{self, Deserialize, Serialize};

pub type r#ParamsCourseids = Vec<Option<i64>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "courseids")]
    pub r#courseids: ParamsCourseids,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItemOptionsItem {
    /// Option name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Whether the option is available or not
    #[serde(rename = "available")]
    pub r#available: Option<bool>,
}

pub type r#ReturnsCoursesItemOptions = Vec<ReturnsCoursesItemOptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCoursesItem {
    /// Course id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    #[serde(rename = "options")]
    pub r#options: ReturnsCoursesItemOptions,
}

/// List of courses
pub type r#ReturnsCourses = Vec<ReturnsCoursesItem>;

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
    /// List of courses
    #[serde(rename = "courses")]
    pub r#courses: ReturnsCourses,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: ReturnsWarnings,
}

pub async fn call<'a>(
    client: &'a mut crate::client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_course_get_user_navigation_options", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}
