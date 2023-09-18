use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// bigbluebuttonbn instance id
    #[serde(rename = "bigbluebuttonbnid")]
    pub r#bigbluebuttonbnid: Option<i64>,
    /// bigbluebuttonbn group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// update cache ?
    #[serde(rename = "updatecache")]
    pub r#updatecache: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPresentationsItem {
    /// presentation URL
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// icon name
    #[serde(rename = "iconname")]
    pub r#iconname: Option<String>,
    /// icon text
    #[serde(rename = "icondesc")]
    pub r#icondesc: Option<String>,
    /// presentation name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}

pub type r#ReturnsPresentations = Vec<ReturnsPresentationsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFeaturesItem {
    /// Feature name.
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Whether the feature is enabled.
    #[serde(rename = "isenabled")]
    pub r#isenabled: Option<bool>,
}

/// List of features for the instance
pub type r#ReturnsFeatures = Vec<ReturnsFeaturesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// CM id
    #[serde(rename = "cmid")]
    pub r#cmid: Option<i64>,
    /// User limit
    #[serde(rename = "userlimit")]
    pub r#userlimit: Option<i64>,
    /// bigbluebuttonbn instance id
    #[serde(rename = "bigbluebuttonbnid")]
    pub r#bigbluebuttonbnid: Option<String>,
    /// bigbluebuttonbn group id
    #[serde(rename = "groupid")]
    pub r#groupid: Option<i64>,
    /// Meeting id
    #[serde(rename = "meetingid")]
    pub r#meetingid: Option<String>,
    /// Opening time
    #[serde(rename = "openingtime")]
    pub r#openingtime: Option<i64>,
    /// Closing time
    #[serde(rename = "closingtime")]
    pub r#closingtime: Option<i64>,
    /// Status running
    #[serde(rename = "statusrunning")]
    pub r#statusrunning: Option<bool>,
    /// Status closed
    #[serde(rename = "statusclosed")]
    pub r#statusclosed: Option<bool>,
    /// Status open
    #[serde(rename = "statusopen")]
    pub r#statusopen: Option<bool>,
    /// Status message
    #[serde(rename = "statusmessage")]
    pub r#statusmessage: Option<String>,
    /// Started at
    #[serde(rename = "startedat")]
    pub r#startedat: Option<i64>,
    /// Moderator count
    #[serde(rename = "moderatorcount")]
    pub r#moderatorcount: Option<i64>,
    /// Participant count
    #[serde(rename = "participantcount")]
    pub r#participantcount: Option<i64>,
    /// Several moderators ?
    #[serde(rename = "moderatorplural")]
    pub r#moderatorplural: Option<bool>,
    /// Several participants ?
    #[serde(rename = "participantplural")]
    pub r#participantplural: Option<bool>,
    /// Can join
    #[serde(rename = "canjoin")]
    pub r#canjoin: Option<bool>,
    /// Is moderator
    #[serde(rename = "ismoderator")]
    pub r#ismoderator: Option<bool>,
    #[serde(rename = "presentations")]
    pub r#presentations: Option<r#ReturnsPresentations>,
    /// Join URL
    #[serde(rename = "joinurl")]
    pub r#joinurl: Option<String>,
    /// Guest access enabled
    #[serde(rename = "guestaccessenabled")]
    pub r#guestaccessenabled: Option<bool>,
    /// Guest URL
    #[serde(rename = "guestjoinurl")]
    pub r#guestjoinurl: Option<String>,
    /// Guest join password
    #[serde(rename = "guestpassword")]
    pub r#guestpassword: Option<String>,
    /// List of features for the instance
    #[serde(rename = "features")]
    pub r#features: Option<r#ReturnsFeatures>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("mod_bigbluebuttonbn_meeting_info", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("mod_bigbluebuttonbn_meeting_info", params)
        .await
}
