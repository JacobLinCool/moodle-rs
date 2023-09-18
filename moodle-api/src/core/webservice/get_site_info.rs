use serde::{self, Deserialize, Serialize};

/// DEPRECATED PARAMETER - it was a design error in the original implementation. \

/// It is ignored now. (parameter kept for backward compatibility)
pub type r#ParamsServiceshortnames = Vec<String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
    /// DEPRECATED PARAMETER - it was a design error in the original implementation. \ It is ignored now. (parameter kept for backward compatibility)
    #[serde(rename = "serviceshortnames")]
    pub r#serviceshortnames: Option<r#ParamsServiceshortnames>,
}

/// functions that are available
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsFunctionsItem {
    /// function name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The version number of the component to which the function belongs
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

pub type r#ReturnsFunctions = Vec<ReturnsFunctionsItem>;

/// Advanced features availability
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnsAdvancedfeaturesItem {
    /// feature name
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// feature value. Usually 1 means enabled.
    #[serde(rename = "value")]
    pub r#value: Option<i64>,
}

/// Advanced features availability
pub type r#ReturnsAdvancedfeatures = Vec<ReturnsAdvancedfeaturesItem>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Returns {
    /// site name
    #[serde(rename = "sitename")]
    pub r#sitename: Option<String>,
    /// username
    #[serde(rename = "username")]
    pub r#username: Option<String>,
    /// first name
    #[serde(rename = "firstname")]
    pub r#firstname: Option<String>,
    /// last name
    #[serde(rename = "lastname")]
    pub r#lastname: Option<String>,
    /// user full name
    #[serde(rename = "fullname")]
    pub r#fullname: Option<String>,
    /// Current language.
    #[serde(rename = "lang")]
    pub r#lang: Option<String>,
    /// user id
    #[serde(rename = "userid")]
    pub r#userid: Option<i64>,
    /// site url
    #[serde(rename = "siteurl")]
    pub r#siteurl: Option<String>,
    /// the user profile picture. Warning: this url is the public URL that only works when forcelogin is set to NO and guestaccess is set to YES. In order to retrieve user profile pictures independently of the Moodle config, replace "pluginfile.php" by "webservice/pluginfile.php?token=WSTOKEN&file=" Of course the user can only see profile picture depending on his/her permissions. Moreover it is recommended to use HTTPS too.
    #[serde(rename = "userpictureurl")]
    pub r#userpictureurl: Option<String>,
    #[serde(rename = "functions")]
    pub r#functions: Option<r#ReturnsFunctions>,
    /// 1 if users are allowed to download files, 0 if not
    #[serde(rename = "downloadfiles")]
    pub r#downloadfiles: Option<i64>,
    /// 1 if users are allowed to upload files, 0 if not
    #[serde(rename = "uploadfiles")]
    pub r#uploadfiles: Option<i64>,
    /// Moodle release number
    #[serde(rename = "release")]
    pub r#release: Option<String>,
    /// Moodle version number
    #[serde(rename = "version")]
    pub r#version: Option<String>,
    /// Mobile custom CSS theme
    #[serde(rename = "mobilecssurl")]
    pub r#mobilecssurl: Option<String>,
    /// Advanced features availability
    #[serde(rename = "advancedfeatures")]
    pub r#advancedfeatures: Option<r#ReturnsAdvancedfeatures>,
    /// true if the user can manage his own files
    #[serde(rename = "usercanmanageownfiles")]
    pub r#usercanmanageownfiles: Option<bool>,
    /// user quota (bytes). 0 means user can ignore the quota
    #[serde(rename = "userquota")]
    pub r#userquota: Option<i64>,
    /// user max upload file size (bytes). -1 means the user can ignore the upload file size
    #[serde(rename = "usermaxuploadfilesize")]
    pub r#usermaxuploadfilesize: Option<i64>,
    /// the default home page for the user: 0 for the site home, 1 for dashboard
    #[serde(rename = "userhomepage")]
    pub r#userhomepage: Option<i64>,
    /// Private user access key for fetching files.
    #[serde(rename = "userprivateaccesskey")]
    pub r#userprivateaccesskey: Option<String>,
    /// Site course ID
    #[serde(rename = "siteid")]
    pub r#siteid: Option<i64>,
    /// Calendar type set in the site.
    #[serde(rename = "sitecalendartype")]
    pub r#sitecalendartype: Option<String>,
    /// Calendar typed used by the user.
    #[serde(rename = "usercalendartype")]
    pub r#usercalendartype: Option<String>,
    /// Whether the user is a site admin or not.
    #[serde(rename = "userissiteadmin")]
    pub r#userissiteadmin: Option<bool>,
    /// Current theme for the user.
    #[serde(rename = "theme")]
    pub r#theme: Option<String>,
    /// Number of concurrent sessions allowed
    #[serde(rename = "limitconcurrentlogins")]
    pub r#limitconcurrentlogins: Option<i64>,
    /// Number of active sessions for current user. Only returned when limitconcurrentlogins is used.
    #[serde(rename = "usersessionscount")]
    pub r#usersessionscount: Option<i64>,
}

pub async fn call<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<Returns> {
    let json = client.post("core_webservice_get_site_info", params).await?;

    serde_json::from_value(json).map_err(|e| e.into())
}

pub async fn call_raw<'a>(
    client: &'a mut moodle_client::MoodleClient,
    params: &'a mut Params,
) -> anyhow::Result<serde_json::Value> {
    client.post("core_webservice_get_site_info", params).await
}
