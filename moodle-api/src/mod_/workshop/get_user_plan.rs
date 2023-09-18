use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Workshop instance id.
    #[serde(rename = "workshopid")]
    pub r#workshopid: Option<i64>,
    /// User id (empty or 0 for current user).
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserplanPhasesItemTasksItem {
    /// Task code.
    #[serde(rename = "code")]
    pub r#code: Option<String>,
    /// Task title.
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Link to task.
    #[serde(rename = "link")]
    pub r#link: Option<String>,
    /// Task details.
    #[serde(rename = "details")]
    pub r#details: Option<String>,
    /// Completion information (maybe empty, maybe a boolean or generic info.
    #[serde(rename = "completed")]
    pub r#completed: Option<String>,
}

pub type r#ReturnsUserplanPhasesItemTasks = Vec<ReturnsUserplanPhasesItemTasksItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserplanPhasesItemActionsItem {
    /// Action type.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Action label.
    #[serde(rename = "label")]
    pub r#label: Option<String>,
    /// Link to action.
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// Get or post.
    #[serde(rename = "method")]
    pub r#method: Option<String>,
}

pub type r#ReturnsUserplanPhasesItemActions = Vec<ReturnsUserplanPhasesItemActionsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserplanPhasesItem {
    /// Phase code.
    #[serde(rename = "code")]
    pub r#code: Option<i64>,
    /// Phase title.
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Whether is the active task.
    #[serde(rename = "active")]
    pub r#active: Option<bool>,
    #[serde(rename = "tasks")]
    pub r#tasks: Option<r#ReturnsUserplanPhasesItemTasks>,
    #[serde(rename = "actions")]
    pub r#actions: Option<r#ReturnsUserplanPhasesItemActions>,
}

pub type r#ReturnsUserplanPhases = Vec<ReturnsUserplanPhasesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserplanExamplesItem {
    /// Example submission id.
    #[serde(rename = "id")]
    pub r#id: Option<i64>,
    /// Example submission title.
    #[serde(rename = "title")]
    pub r#title: Option<String>,
    /// Example submission assessment id.
    #[serde(rename = "assessmentid")]
    pub r#assessmentid: Option<i64>,
    /// The submission grade.
    #[serde(rename = "grade")]
    pub r#grade: Option<f64>,
    /// The assessment grade.
    #[serde(rename = "gradinggrade")]
    pub r#gradinggrade: Option<f64>,
}

pub type r#ReturnsUserplanExamples = Vec<ReturnsUserplanExamplesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsUserplan {
    #[serde(rename = "phases")]
    pub r#phases: Option<r#ReturnsUserplanPhases>,
    #[serde(rename = "examples")]
    pub r#examples: Option<r#ReturnsUserplanExamples>,
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
    #[serde(rename = "userplan")]
    pub r#userplan: Option<ReturnsUserplan>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("mod_workshop_get_user_plan", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("mod_workshop_get_user_plan", params).await
}
