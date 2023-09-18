use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Assessment id
    #[serde(rename = "assessmentid")]
    pub r#assessmentid: Option<i64>,
    /// The form mode (assessment or preview)
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
}

/// File.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDescriptionfilesItem {
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

/// Files in the description text
pub type r#ReturnsDescriptionfiles = Vec<ReturnsDescriptionfilesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsOptionsItem {
    /// Option name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Option value.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// The form options.
pub type r#ReturnsOptions = Vec<ReturnsOptionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFieldsItem {
    /// Field name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Field default value.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// The form fields.
pub type r#ReturnsFields = Vec<ReturnsFieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsCurrentItem {
    /// Field name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Current field value.
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// The current field values.
pub type r#ReturnsCurrent = Vec<ReturnsCurrentItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsDimensionsinfoItem {
    /// Dimension id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Minimum grade for the dimension.
    #[serde(rename = "min")]
    pub r#min: Option<f64>,
    /// Maximum grade for the dimension.
    #[serde(rename = "max")]
    pub r#max: Option<f64>,
    /// The weight of the dimension.
    #[serde(rename = "weight")]
    pub r#weight: Option<String>,
    /// Scale items (if used).
    #[serde(rename = "scale")]
    pub r#scale: Option<String>,
}

/// The dimensions general information.
pub type r#ReturnsDimensionsinfo = Vec<ReturnsDimensionsinfoItem>;

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
    /// The number of dimenssions used by the form.
    #[serde(rename = "dimenssionscount")]
    pub r#dimenssionscount: Option<i64>,
    /// Files in the description text
    #[serde(rename = "descriptionfiles")]
    pub r#descriptionfiles: Option<r#ReturnsDescriptionfiles>,
    /// The form options.
    #[serde(rename = "options")]
    pub r#options: Option<r#ReturnsOptions>,
    /// The form fields.
    #[serde(rename = "fields")]
    pub r#fields: Option<r#ReturnsFields>,
    /// The current field values.
    #[serde(rename = "current")]
    pub r#current: Option<r#ReturnsCurrent>,
    /// The dimensions general information.
    #[serde(rename = "dimensionsinfo")]
    pub r#dimensionsinfo: Option<r#ReturnsDimensionsinfo>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_workshop_get_assessment_form_definition", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_workshop_get_assessment_form_definition", params)
        .await
}
