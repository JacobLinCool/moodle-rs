use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// User name
    #[serde(rename = "username")]
    pub r#username: Option<String>,
    /// User email
    #[serde(rename = "email")]
    pub r#email: Option<String>,
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
    /// The returned status of the process: dataerror: Error in the sent data (username or email). More information in warnings field. emailpasswordconfirmmaybesent: Email sent or not (depends on user found in database). emailpasswordconfirmnotsent: Failure, user not found. emailpasswordconfirmnoemail: Failure, email not found. emailalreadysent: Email already sent. emailpasswordconfirmsent: User pending confirmation. emailresetconfirmsent: Email sent.
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// Important information for the user about the process.
    #[serde(rename = "notice")]
    pub r#notice: Option<String>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_auth_request_password_reset", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_auth_request_password_reset", params)
        .await
}
