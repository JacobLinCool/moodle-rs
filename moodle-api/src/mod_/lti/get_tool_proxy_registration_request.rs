use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Tool proxy id
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// LTI message type
    #[serde(rename = "lti_message_type")]
    pub r#lti_message_type: Option<String>,
    /// LTI version
    #[serde(rename = "lti_version")]
    pub r#lti_version: Option<String>,
    /// Tool proxy registration key
    #[serde(rename = "reg_key")]
    pub r#reg_key: Option<String>,
    /// Tool proxy registration password
    #[serde(rename = "reg_password")]
    pub r#reg_password: Option<String>,
    /// Tool proxy registration url
    #[serde(rename = "reg_url")]
    pub r#reg_url: Option<String>,
    /// Tool consumers profile URL
    #[serde(rename = "tc_profile_url")]
    pub r#tc_profile_url: Option<String>,
    /// URL to redirect on registration completion
    #[serde(rename = "launch_presentation_return_url")]
    pub r#launch_presentation_return_url: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_lti_get_tool_proxy_registration_request", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_lti_get_tool_proxy_registration_request", params)
        .await
}
