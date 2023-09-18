use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The data from the event form
    #[serde(rename = "formdata")]
    pub r#formdata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// True if the user's enrolment was successfully updated
    #[serde(rename = "result")]
    pub r#result: Option<bool>,
    /// Indicates invalid form data
    #[serde(rename = "validationerror")]
    pub r#validationerror: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_enrol_submit_user_enrolment_form", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_enrol_submit_user_enrolment_form", params)
        .await
}
