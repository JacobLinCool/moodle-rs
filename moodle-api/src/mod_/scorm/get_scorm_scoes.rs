use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// scorm instance id
    #[serde(rename = "scormid")]
    pub r#scormid: Option<i64>,
    /// organization id
    #[serde(rename = "organization")]
    pub r#organization: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsScoesItemExtradataItem {
    /// element name
    #[serde(rename = "element")]
    pub r#element: Option<String>,
    /// element value
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// Additional SCO data
pub type r#ReturnsScoesItemExtradata = Vec<ReturnsScoesItemExtradataItem>;

/// SCORM SCO data
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsScoesItem {
    /// sco id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// scorm id
    #[serde(rename = "scorm")]
    pub r#scorm: Option<i64>,
    /// manifest id
    #[serde(rename = "manifest")]
    pub r#manifest: Option<String>,
    /// organization id
    #[serde(rename = "organization")]
    pub r#organization: Option<String>,
    /// parent
    #[serde(rename = "parent")]
    pub r#parent: Option<String>,
    /// identifier
    #[serde(rename = "identifier")]
    pub r#identifier: Option<String>,
    /// launch file
    #[serde(rename = "launch")]
    pub r#launch: Option<String>,
    /// scorm type (asset, sco)
    #[serde(rename = "scormtype")]
    pub r#scormtype: Option<String>,
    /// sco title
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// sort order
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// Additional SCO data
    #[serde(rename = "extradata")]
    pub r#extradata: Option<r#ReturnsScoesItemExtradata>,
}

pub type r#ReturnsScoes = Vec<ReturnsScoesItem>;

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
    #[serde(rename = "scoes")]
    pub r#scoes: Option<r#ReturnsScoes>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_scorm_get_scorm_scoes", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_scorm_get_scorm_scoes", params).await
}
