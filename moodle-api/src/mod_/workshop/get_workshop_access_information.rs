use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// Workshop instance id.
    #[serde(rename = "workshopid")]
    pub r#workshopid: Option<i64>,
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
    /// Is the given user allowed to create their submission?
    #[serde(rename = "creatingsubmissionallowed")]
    pub r#creatingsubmissionallowed: Option<bool>,
    /// Is the user allowed to modify his existing submission?
    #[serde(rename = "modifyingsubmissionallowed")]
    pub r#modifyingsubmissionallowed: Option<bool>,
    /// Is the user allowed to create/edit his assessments?
    #[serde(rename = "assessingallowed")]
    pub r#assessingallowed: Option<bool>,
    /// Are reviewers allowed to create/edit their assessments of the example submissions?.
    #[serde(rename = "assessingexamplesallowed")]
    pub r#assessingexamplesallowed: Option<bool>,
    /// Whether the given user has assessed all his required examples before submission (always true if there are not examples to assess or not configured to check before submission).
    #[serde(rename = "examplesassessedbeforesubmission")]
    pub r#examplesassessedbeforesubmission: Option<bool>,
    /// Whether the given user has assessed all his required examples before assessment (always true if there are not examples to assessor not configured to check before assessment).
    #[serde(rename = "examplesassessedbeforeassessment")]
    pub r#examplesassessedbeforeassessment: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
    /// Whether the user has the capability mod/workshop:view allowed.
    #[serde(rename = "canview")]
    pub r#canview: Option<bool>,
    /// Whether the user has the capability mod/workshop:addinstance allowed.
    #[serde(rename = "canaddinstance")]
    pub r#canaddinstance: Option<bool>,
    /// Whether the user has the capability mod/workshop:switchphase allowed.
    #[serde(rename = "canswitchphase")]
    pub r#canswitchphase: Option<bool>,
    /// Whether the user has the capability mod/workshop:editdimensions allowed.
    #[serde(rename = "caneditdimensions")]
    pub r#caneditdimensions: Option<bool>,
    /// Whether the user has the capability mod/workshop:submit allowed.
    #[serde(rename = "cansubmit")]
    pub r#cansubmit: Option<bool>,
    /// Whether the user has the capability mod/workshop:peerassess allowed.
    #[serde(rename = "canpeerassess")]
    pub r#canpeerassess: Option<bool>,
    /// Whether the user has the capability mod/workshop:manageexamples allowed.
    #[serde(rename = "canmanageexamples")]
    pub r#canmanageexamples: Option<bool>,
    /// Whether the user has the capability mod/workshop:allocate allowed.
    #[serde(rename = "canallocate")]
    pub r#canallocate: Option<bool>,
    /// Whether the user has the capability mod/workshop:publishsubmissions allowed.
    #[serde(rename = "canpublishsubmissions")]
    pub r#canpublishsubmissions: Option<bool>,
    /// Whether the user has the capability mod/workshop:viewauthornames allowed.
    #[serde(rename = "canviewauthornames")]
    pub r#canviewauthornames: Option<bool>,
    /// Whether the user has the capability mod/workshop:viewreviewernames allowed.
    #[serde(rename = "canviewreviewernames")]
    pub r#canviewreviewernames: Option<bool>,
    /// Whether the user has the capability mod/workshop:viewallsubmissions allowed.
    #[serde(rename = "canviewallsubmissions")]
    pub r#canviewallsubmissions: Option<bool>,
    /// Whether the user has the capability mod/workshop:viewpublishedsubmissions allowed.
    #[serde(rename = "canviewpublishedsubmissions")]
    pub r#canviewpublishedsubmissions: Option<bool>,
    /// Whether the user has the capability mod/workshop:viewauthorpublished allowed.
    #[serde(rename = "canviewauthorpublished")]
    pub r#canviewauthorpublished: Option<bool>,
    /// Whether the user has the capability mod/workshop:viewallassessments allowed.
    #[serde(rename = "canviewallassessments")]
    pub r#canviewallassessments: Option<bool>,
    /// Whether the user has the capability mod/workshop:overridegrades allowed.
    #[serde(rename = "canoverridegrades")]
    pub r#canoverridegrades: Option<bool>,
    /// Whether the user has the capability mod/workshop:ignoredeadlines allowed.
    #[serde(rename = "canignoredeadlines")]
    pub r#canignoredeadlines: Option<bool>,
    /// Whether the user has the capability mod/workshop:deletesubmissions allowed.
    #[serde(rename = "candeletesubmissions")]
    pub r#candeletesubmissions: Option<bool>,
    /// Whether the user has the capability mod/workshop:exportsubmissions allowed.
    #[serde(rename = "canexportsubmissions")]
    pub r#canexportsubmissions: Option<bool>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_workshop_get_workshop_access_information", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_workshop_get_workshop_access_information", params)
        .await
}
