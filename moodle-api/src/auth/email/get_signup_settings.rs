use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {}

pub type r#ReturnsNamefields = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsProfilefieldsItem {
    /// Profile field id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Profile field shortname
    #[serde(rename = "shortname")]
    pub r#shortname: Option<String>,
    /// Profield field name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Profield field datatype
    #[serde(rename = "datatype")]
    pub r#datatype: Option<String>,
    /// Profield field description
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// description format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "descriptionformat")]
    pub r#descriptionformat: Option<i64>,
    /// Profield field category id
    #[serde(rename = "categoryid")]
    pub r#categoryid: Option<i64>,
    /// Profield field category name
    #[serde(rename = "categoryname")]
    pub r#categoryname: Option<String>,
    /// Profield field sort order
    #[serde(rename = "sortorder")]
    pub r#sortorder: Option<i64>,
    /// Profield field required
    #[serde(rename = "required")]
    pub r#required: Option<i64>,
    /// Profield field locked
    #[serde(rename = "locked")]
    pub r#locked: Option<i64>,
    /// Profield field visible
    #[serde(rename = "visible")]
    pub r#visible: Option<i64>,
    /// Profield field unique
    #[serde(rename = "forceunique")]
    pub r#forceunique: Option<i64>,
    /// Profield field in signup form
    #[serde(rename = "signup")]
    pub r#signup: Option<i64>,
    /// Profield field default data
    #[serde(rename = "defaultdata")]
    pub r#defaultdata: Option<String>,
    /// defaultdata format (1 = HTML, 0 = MOODLE, 2 = PLAIN, or 4 = MARKDOWN
    #[serde(rename = "defaultdataformat")]
    pub r#defaultdataformat: Option<i64>,
    /// Profield field settings
    #[serde(rename = "param1")]
    pub r#param1: Option<String>,
    /// Profield field settings
    #[serde(rename = "param2")]
    pub r#param2: Option<String>,
    /// Profield field settings
    #[serde(rename = "param3")]
    pub r#param3: Option<String>,
    /// Profield field settings
    #[serde(rename = "param4")]
    pub r#param4: Option<String>,
    /// Profield field settings
    #[serde(rename = "param5")]
    pub r#param5: Option<String>,
}

/// Required profile fields
pub type r#ReturnsProfilefields = Vec<ReturnsProfilefieldsItem>;

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
    #[serde(rename = "namefields")]
    pub r#namefields: Option<r#ReturnsNamefields>,
    /// Password policy
    #[serde(rename = "passwordpolicy")]
    pub r#passwordpolicy: Option<String>,
    /// Site policy
    #[serde(rename = "sitepolicy")]
    pub r#sitepolicy: Option<String>,
    /// Site policy handler
    #[serde(rename = "sitepolicyhandler")]
    pub r#sitepolicyhandler: Option<String>,
    /// Default city
    #[serde(rename = "defaultcity")]
    pub r#defaultcity: Option<String>,
    /// Default country
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// Required profile fields
    #[serde(rename = "profilefields")]
    pub r#profilefields: Option<r#ReturnsProfilefields>,
    /// Recaptcha public key
    #[serde(rename = "recaptchapublickey")]
    pub r#recaptchapublickey: Option<String>,
    /// Recaptcha challenge hash
    #[serde(rename = "recaptchachallengehash")]
    pub r#recaptchachallengehash: Option<String>,
    /// Recaptcha challenge noscript image
    #[serde(rename = "recaptchachallengeimage")]
    pub r#recaptchachallengeimage: Option<String>,
    /// Recaptcha challenge js url
    #[serde(rename = "recaptchachallengejs")]
    pub r#recaptchachallengejs: Option<String>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("auth_email_get_signup_settings", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("auth_email_get_signup_settings", params).await
}
