use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The data to create the purpose, encoded as a json array
    #[serde(rename = "jsonformdata")]
    pub r#jsonformdata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPurposeFormattedlawfulbasesItem {
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
}

/// formattedlawfulbases
pub type r#ReturnsPurposeFormattedlawfulbases = Vec<ReturnsPurposeFormattedlawfulbasesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPurposeFormattedsensitivedatareasonsItem {
    /// name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
}

/// formattedsensitivedatareasons
pub type r#ReturnsPurposeFormattedsensitivedatareasons =
    Vec<ReturnsPurposeFormattedsensitivedatareasonsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPurpose {
    /// The purpose name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The purpose description.
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// Comma-separated IDs matching records in tool_dataprivacy_lawfulbasis.
    #[serde(rename = "lawfulbases")]
    pub r#lawfulbases: Option<String>,
    /// Comma-separated IDs matching records in tool_dataprivacy_sensitive
    #[serde(rename = "sensitivedatareasons")]
    pub r#sensitivedatareasons: Option<String>,
    /// Retention period. ISO_8601 durations format (as in DateInterval format).
    #[serde(rename = "retentionperiod")]
    pub r#retentionperiod: Option<String>,
    /// Data retention with higher precedent over user's request to be forgotten.
    #[serde(rename = "protected")]
    pub r#protected: Option<i64>,
    /// id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// timecreated
    #[serde(rename = "timecreated")]
    pub r#timecreated: Option<i64>,
    /// timemodified
    #[serde(rename = "timemodified")]
    pub r#timemodified: Option<i64>,
    /// usermodified
    #[serde(rename = "usermodified")]
    pub r#usermodified: Option<i64>,
    /// formattedretentionperiod
    #[serde(rename = "formattedretentionperiod")]
    pub r#formattedretentionperiod: Option<String>,
    /// formattedlawfulbases
    #[serde(rename = "formattedlawfulbases")]
    pub r#formattedlawfulbases: Option<r#ReturnsPurposeFormattedlawfulbases>,
    /// formattedsensitivedatareasons
    #[serde(rename = "formattedsensitivedatareasons")]
    pub r#formattedsensitivedatareasons: Option<r#ReturnsPurposeFormattedsensitivedatareasons>,
    /// roleoverrides
    #[serde(rename = "roleoverrides")]
    pub r#roleoverrides: Option<String>,
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
    #[serde(rename = "purpose")]
    pub r#purpose: Option<ReturnsPurpose>,
    /// Were there validation errors
    #[serde(rename = "validationerrors")]
    pub r#validationerrors: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_dataprivacy_create_purpose_form", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_dataprivacy_create_purpose_form", params)
        .await
}
