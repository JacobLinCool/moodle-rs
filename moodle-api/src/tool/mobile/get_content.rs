use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsArgsItem {
    /// Param name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Param value.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Args for the method are optional.
pub type r#ParamsArgs = Vec<ParamsArgsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Component where the class is e.g. mod_assign.
    #[serde(rename = "component")]
    pub r#component: Option<String>,
    /// Method to execute in class \$component\output\mobile.
    #[serde(rename = "method")]
    pub r#method: Option<String>,
    /// Args for the method are optional.
    #[serde(rename = "args")]
    pub r#args: Option<r#ParamsArgs>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTemplatesItem {
    /// ID of the template.
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// HTML code.
    #[serde(rename = "html")]
    pub r#html: Option<String>,
}

/// Templates required by the generated content.
pub type r#ReturnsTemplates = Vec<ReturnsTemplatesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsOtherdataItem {
    /// Field name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Field value.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Other data that can be used or manipulated by the template via 2-way data-binding.
pub type r#ReturnsOtherdata = Vec<ReturnsOtherdataItem>;

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFilesItem {
    /// File name.
    #[serde(rename = "filename")]
    pub r#filename: Option<String>,
    /// File path.
    #[serde(rename = "filepath")]
    pub r#filepath: Option<String>,
    /// File size.
    #[serde(rename = "filesize")]
    pub r#filesize: Option<i64>,
    /// Downloadable file url.
    #[serde(rename = "fileurl")]
    pub r#fileurl: Option<String>,
    /// Time modified.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// File mime type.
    #[serde(rename = "mimetype")]
    pub r#mimetype: Option<String>,
    /// Whether is an external file.
    #[serde(rename = "isexternalfile")]
    pub r#isexternalfile: Option<bool>,
    /// The repository type for external files.
    #[serde(rename = "repositorytype")]
    pub r#repositorytype: Option<String>,
}

/// Files in the content.
pub type r#ReturnsFiles = Vec<ReturnsFilesItem>;

/// List of allowed users.
pub type r#ReturnsRestrictUsers = Vec<i64>;

/// List of allowed courses.
pub type r#ReturnsRestrictCourses = Vec<i64>;

/// Restrict this content to certain users or courses.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsRestrict {
    /// List of allowed users.
    #[serde(rename = "users")]
    pub r#users: Option<r#ReturnsRestrictUsers>,
    /// List of allowed courses.
    #[serde(rename = "courses")]
    pub r#courses: Option<r#ReturnsRestrictCourses>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Templates required by the generated content.
    #[serde(rename = "templates")]
    pub r#templates: Option<r#ReturnsTemplates>,
    /// JavaScript code.
    #[serde(rename = "javascript")]
    pub r#javascript: Option<String>,
    /// Other data that can be used or manipulated by the template via 2-way data-binding.
    #[serde(rename = "otherdata")]
    pub r#otherdata: Option<r#ReturnsOtherdata>,
    /// Files in the content.
    #[serde(rename = "files")]
    pub r#files: Option<r#ReturnsFiles>,
    /// Restrict this content to certain users or courses.
    #[serde(rename = "restrict")]
    pub r#restrict: Option<ReturnsRestrict>,
    /// Whether we consider this disabled or not.
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("tool_mobile_get_content", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("tool_mobile_get_content", params).await
}
