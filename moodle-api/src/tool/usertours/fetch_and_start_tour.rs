use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Tour ID
    #[serde(rename = "tourid")]
    pub r#tourid: Option<i64>,
    /// Context ID
    #[serde(rename = "context")]
    pub r#context: Option<i64>,
    /// Page URL
    #[serde(rename = "pageurl")]
    pub r#pageurl: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTourconfigStepsItem {
    /// Step Title
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Step Content
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// Step Target
    #[serde(rename = "element")]
    pub r#element: Option<String>,
    /// Step Placement
    #[serde(rename = "placement")]
    pub r#placement: Option<String>,
    /// Delay before showing the step (ms)
    #[serde(rename = "delay")]
    pub r#delay: Option<i64>,
    /// Whether a backdrop should be used
    #[serde(rename = "backdrop")]
    pub r#backdrop: Option<bool>,
    /// Whether to move to the next step when the target element is clicked
    #[serde(rename = "reflex")]
    pub r#reflex: Option<bool>,
    /// Whether to display the step even if it could not be found
    #[serde(rename = "orphan")]
    pub r#orphan: Option<bool>,
    /// The actual ID of the step
    #[serde(rename = "stepid")]
    pub r#stepid: Option<i64>,
}

pub type r#ReturnsTourconfigSteps = Vec<ReturnsTourconfigStepsItem>;

/// Tour config
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsTourconfig {
    /// Tour Name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[serde(rename = "steps")]
    pub r#steps: Option<r#ReturnsTourconfigSteps>,
    /// Label of the end tour button
    #[serde(rename = "endtourlabel")]
    pub r#endtourlabel: Option<String>,
    /// display step number
    #[serde(rename = "displaystepnumbers")]
    pub r#displaystepnumbers: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// Tour config
    #[serde(rename = "tourconfig")]
    pub r#tourconfig: Option<ReturnsTourconfig>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("tool_usertours_fetch_and_start_tour", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("tool_usertours_fetch_and_start_tour", params)
        .await
}
