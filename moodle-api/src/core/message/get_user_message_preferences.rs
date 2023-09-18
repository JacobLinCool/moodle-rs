use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// id of the user, 0 for current user
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreferencesProcessorsItem {
    /// Display name
    #[serde(rename = "displayname")]
    pub r#displayname: Option<String>,
    /// Processor name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Whether has settings
    #[serde(rename = "hassettings")]
    pub r#hassettings: Option<bool>,
    /// Context id
    #[serde(rename = "contextid")]
    pub r#contextid: Option<i64>,
    /// Whether is configured by the user
    #[serde(rename = "userconfigured")]
    pub r#userconfigured: Option<i64>,
}

/// Config form values
pub type r#ReturnsPreferencesProcessors = Vec<ReturnsPreferencesProcessorsItem>;

/// DEPRECATED ATTRIBUTE -

/// Kept for backward compatibility, use enabled instead.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreferencesComponentsItemNotificationsItemProcessorsItemLoggedin {
    /// Name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Display name
    #[serde(rename = "displayname")]
    pub r#displayname: Option<String>,
    /// Is checked?
    #[serde(rename = "checked")]
    pub r#checked: Option<bool>,
}

/// DEPRECATED ATTRIBUTE -

/// Kept for backward compatibility, use enabled instead.
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreferencesComponentsItemNotificationsItemProcessorsItemLoggedoff {
    /// Name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Display name
    #[serde(rename = "displayname")]
    pub r#displayname: Option<String>,
    /// Is checked?
    #[serde(rename = "checked")]
    pub r#checked: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreferencesComponentsItemNotificationsItemProcessorsItem {
    /// Display name
    #[serde(rename = "displayname")]
    pub r#displayname: Option<String>,
    /// Processor name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Is locked by admin?
    #[serde(rename = "locked")]
    pub r#locked: Option<bool>,
    /// Text to display if locked
    #[serde(rename = "lockedmessage")]
    pub r#lockedmessage: Option<String>,
    /// Is configured?
    #[serde(rename = "userconfigured")]
    pub r#userconfigured: Option<i64>,
    /// DEPRECATED ATTRIBUTE - Kept for backward compatibility, use enabled instead.
    #[serde(rename = "loggedin")]
    pub r#loggedin: Option<ReturnsPreferencesComponentsItemNotificationsItemProcessorsItemLoggedin>,
    /// DEPRECATED ATTRIBUTE - Kept for backward compatibility, use enabled instead.
    #[serde(rename = "loggedoff")]
    pub r#loggedoff:
        Option<ReturnsPreferencesComponentsItemNotificationsItemProcessorsItemLoggedoff>,
    /// Is enabled?
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
}

/// Processors values for this notification
pub type r#ReturnsPreferencesComponentsItemNotificationsItemProcessors =
    Vec<ReturnsPreferencesComponentsItemNotificationsItemProcessorsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreferencesComponentsItemNotificationsItem {
    /// Display name
    #[serde(rename = "displayname")]
    pub r#displayname: Option<String>,
    /// Preference key
    #[serde(rename = "preferencekey")]
    pub r#preferencekey: Option<String>,
    /// Processors values for this notification
    #[serde(rename = "processors")]
    pub r#processors: Option<r#ReturnsPreferencesComponentsItemNotificationsItemProcessors>,
}

/// List of notificaitons for the component
pub type r#ReturnsPreferencesComponentsItemNotifications =
    Vec<ReturnsPreferencesComponentsItemNotificationsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreferencesComponentsItem {
    /// Display name
    #[serde(rename = "displayname")]
    pub r#displayname: Option<String>,
    /// List of notificaitons for the component
    #[serde(rename = "notifications")]
    pub r#notifications: Option<r#ReturnsPreferencesComponentsItemNotifications>,
}

/// Available components
pub type r#ReturnsPreferencesComponents = Vec<ReturnsPreferencesComponentsItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsPreferences {
    /// User id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// Whether all the preferences are disabled
    #[serde(rename = "disableall")]
    pub r#disableall: Option<i64>,
    /// Config form values
    #[serde(rename = "processors")]
    pub r#processors: Option<r#ReturnsPreferencesProcessors>,
    /// Available components
    #[serde(rename = "components")]
    pub r#components: Option<r#ReturnsPreferencesComponents>,
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
    #[serde(rename = "preferences")]
    pub r#preferences: Option<ReturnsPreferences>,
    /// Privacy messaging setting to define who can message you
    #[serde(rename = "blocknoncontacts")]
    pub r#blocknoncontacts: Option<i64>,
    /// User preference for using enter to send messages
    #[serde(rename = "entertosend")]
    pub r#entertosend: Option<bool>,
    /// list of warnings
    #[serde(rename = "warnings")]
    pub r#warnings: Option<r#ReturnsWarnings>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client
        .post("core_message_get_user_message_preferences", params)
        .await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client
        .post("core_message_get_user_message_preferences", params)
        .await
}
