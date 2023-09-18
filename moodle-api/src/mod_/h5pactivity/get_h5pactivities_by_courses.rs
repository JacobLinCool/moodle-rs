use serde::{self, Deserialize, Serialize};

/// Array of course ids
pub type r#ParamsCourseids = Vec<i64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Array of course ids
    #[serde(rename = "courseids")]
    pub r#courseids: Option<r#ParamsCourseids>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsH5pactivitiesItemIntrofilesItem {
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
    /// The repository type for the external files.
    #[serde(rename = "repositorytype")]
    pub r#repositorytype: Option<String>,
}

/// introfiles
pub type r#ReturnsH5pactivitiesItemIntrofiles = Vec<ReturnsH5pactivitiesItemIntrofilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsH5pactivitiesItemPackageItem {
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
    /// The repository type for the external files.
    #[serde(rename = "repositorytype")]
    pub r#repositorytype: Option<String>,
}

/// package
pub type r#ReturnsH5pactivitiesItemPackage = Vec<ReturnsH5pactivitiesItemPackageItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsH5pactivitiesItemDeployedfile {
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsH5pactivitiesItem {
    /// The primary key of the record.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Course id this h5p activity is part of.
    #[serde(rename = "course")]
    pub r#course: Option<i64>,
    /// The name of the activity module instance.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Timestamp of when the instance was added to the course.
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// Timestamp of when the instance was last modified.
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// H5P activity description.
    #[serde(rename = "intro")]
    pub r#intro: Option<String>,
    /// intro format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "introformat")]
    pub r#introformat: Option<i64>,
    /// The maximum grade for submission.
    #[serde(rename = "grade")]
    pub r#grade: Option<i64>,
    /// H5P Button display options.
    #[serde(rename = "displayoptions")]
    pub r#displayoptions: Option<i64>,
    /// Enable xAPI tracking.
    #[serde(rename = "enabletracking")]
    pub r#enabletracking: Option<i64>,
    /// Which H5P attempt is used for grading.
    #[serde(rename = "grademethod")]
    pub r#grademethod: Option<i64>,
    /// Sha1 hash of file content.
    #[serde(rename = "contenthash")]
    pub r#contenthash: Option<String>,
    /// coursemodule
    #[serde(rename = "coursemodule")]
    pub r#coursemodule: Option<i64>,
    /// context
    #[serde(rename = "context")]
    pub r#context: Option<i64>,
    /// introfiles
    #[serde(rename = "introfiles")]
    pub r#introfiles: Option<r#ReturnsH5pactivitiesItemIntrofiles>,
    /// package
    #[serde(rename = "package")]
    pub r#package: Option<r#ReturnsH5pactivitiesItemPackage>,
    #[serde(rename = "deployedfile")]
    pub r#deployedfile: Option<ReturnsH5pactivitiesItemDeployedfile>,
}

pub type r#ReturnsH5pactivities = Vec<ReturnsH5pactivitiesItem>;

/// H5P global settings
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsH5pglobalsettings {
    /// Whether saving state is enabled.
    #[serde(rename = "enablesavestate")]
    pub r#enablesavestate: Option<bool>,
    /// How often (in seconds) state is saved.
    #[serde(rename = "savestatefreq")]
    pub r#savestatefreq: Option<i64>,
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
    #[serde(rename = "h5pactivities")]
    pub r#h5pactivities: Option<r#ReturnsH5pactivities>,
    /// H5P global settings
    #[serde(rename = "h5pglobalsettings")]
    pub r#h5pglobalsettings: Option<ReturnsH5pglobalsettings>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_h5pactivity_get_h5pactivities_by_courses", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_h5pactivity_get_h5pactivities_by_courses", params)
        .await
}
