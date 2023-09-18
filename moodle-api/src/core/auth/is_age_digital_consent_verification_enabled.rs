use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// True if digital consent verification is enabled, false otherwise.
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post(
            "core_auth_is_age_digital_consent_verification_enabled",
            params,
        )
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post(
            "core_auth_is_age_digital_consent_verification_enabled",
            params,
        )
        .await
}
