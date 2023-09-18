use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Form class
    #[serde(rename = "form")]
    pub r#form: Option<String>,
    /// url-encoded form data
    #[serde(rename = "formdata")]
    pub r#formdata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// If form was submitted and validated
    #[serde(rename = "submitted")]
    pub r#submitted: Option<bool>,
    /// JSON-encoded return data from form processing method
    #[serde(rename = "data")]
    pub r#data: Option<String>,
    /// HTML fragment of the form
    #[serde(rename = "html")]
    pub r#html: Option<String>,
    /// JavaScript fragment of the form
    #[serde(rename = "javascript")]
    pub r#javascript: Option<String>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_form_dynamic_form", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_form_dynamic_form", params).await
}
