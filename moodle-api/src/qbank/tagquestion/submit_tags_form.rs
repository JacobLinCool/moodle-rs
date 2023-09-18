use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// The question id
    #[serde(rename = "questionid")]
    pub r#questionid: Option<i64>,
    /// The editing context id
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// The data from the tag form
    #[serde(rename = "formdata")]
    pub r#formdata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// status: true if success
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("qbank_tagquestion_submit_tags_form", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("qbank_tagquestion_submit_tags_form", params)
        .await
}
