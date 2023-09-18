use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    #[serde(rename = "slotid")]
    pub r#slotid: Option<i64>,
    #[serde(rename = "newversion")]
    pub r#newversion: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    #[serde(rename = "result")]
    pub r#result: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_quiz_set_question_version", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_quiz_set_question_version", params).await
}
