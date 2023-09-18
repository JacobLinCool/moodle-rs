use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParamsCustomprofilefieldsItem {
    /// The type of the custom field
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The name of the custom field
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Custom field value, can be an encoded json if required
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}

/// User custom fields (also known as user profile fields)
pub type r#ParamsCustomprofilefields = Vec<ParamsCustomprofilefieldsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Username
    #[serde(rename = "username")]
    pub r#username: Option<String>,
    /// Plain text password
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// The first name(s) of the user
    #[serde(rename = "firstname")]
    pub r#firstname: Option<String>,
    /// The family name of the user
    #[serde(rename = "lastname")]
    pub r#lastname: Option<String>,
    /// A valid and unique email address
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// Home city of the user
    #[serde(rename = "city")]
    pub r#city: Option<String>,
    /// Home country code
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// Recaptcha challenge hash
    #[serde(rename = "recaptchachallengehash")]
    pub r#recaptchachallengehash: Option<String>,
    /// Recaptcha response
    #[serde(rename = "recaptcharesponse")]
    pub r#recaptcharesponse: Option<String>,
    /// User custom fields (also known as user profile fields)
    #[serde(rename = "customprofilefields")]
    pub r#customprofilefields: Option<r#ParamsCustomprofilefields>,
    /// Redirect the user to this site url after confirmation.
    #[serde(rename = "redirect")]
    pub r#redirect: Option<String>,
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
    /// True if the user was created false otherwise
    #[serde(rename = "success")]
    pub r#success: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("auth_email_signup_user", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("auth_email_signup_user", params).await
}
